use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use regex::bytes::Regex;
use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatchRule {
    pub id: String,
    pub regex: String,
    pub payload: String,
    #[serde(default = "default_hits")]
    pub required_hits: u32,
}

fn default_hits() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchReport {
    pub patches_attempted: u32,
    pub patches_hit: u32,
    pub files_modified: Vec<PathBuf>,
    pub per_patch: BTreeMap<String, u32>,
}

/// 在 unpacked_dir 下所有 .js 文件中应用一组 patches。
/// 倒序插入保证 offset 不错位。任何 patch 命中数 < required_hits 即返回 PatchNoHit。
pub fn apply_patches(unpacked_dir: &Path, patches: &[PatchRule]) -> Result<PatchReport> {
    let compiled: Vec<(Regex, &PatchRule)> = patches
        .iter()
        .map(|p| {
            Regex::new(&p.regex)
                .map(|r| (r, p))
                .map_err(|e| AppError::Internal(format!("正则编译失败 {}: {e}", p.id)))
        })
        .collect::<Result<Vec<_>>>()?;

    // 收集所有 .js 文件中各 patch 的命中位置
    // file -> Vec<(offset, payload_bytes)>
    let mut per_file_hits: BTreeMap<PathBuf, Vec<(usize, Vec<u8>)>> = BTreeMap::new();
    // 全局命中计数
    let mut per_patch_hits: BTreeMap<String, u32> = BTreeMap::new();
    for p in patches {
        per_patch_hits.insert(p.id.clone(), 0);
    }

    for entry in walkdir_js(unpacked_dir)? {
        let bytes = fs::read(&entry).map_err(AppError::Io)?;
        for (re, rule) in &compiled {
            for m in re.find_iter(&bytes) {
                per_file_hits
                    .entry(entry.clone())
                    .or_default()
                    .push((m.start(), rule.payload.as_bytes().to_vec()));
                *per_patch_hits.get_mut(&rule.id).unwrap() += 1;
            }
        }
    }

    // 校验每条 required_hits
    for p in patches {
        let hits = per_patch_hits[&p.id];
        if hits < p.required_hits {
            return Err(AppError::PatchNoHit {
                patch_id: p.id.clone(),
            });
        }
    }

    // 倒序插入
    let mut report = PatchReport {
        patches_attempted: patches.len() as u32,
        patches_hit: patches.len() as u32,
        files_modified: Vec::new(),
        per_patch: per_patch_hits,
    };
    for (path, mut anchors) in per_file_hits {
        anchors.sort_by_key(|a| std::cmp::Reverse(a.0));
        let mut content = fs::read(&path).map_err(AppError::Io)?;
        for (offset, payload) in anchors {
            content.splice(offset..offset, payload.iter().copied());
        }
        fs::write(&path, &content).map_err(AppError::Io)?;
        report.files_modified.push(path);
    }
    Ok(report)
}

fn walkdir_js(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    walk(dir, &mut out)?;
    Ok(out)
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(AppError::Io)? {
        let entry = entry.map_err(AppError::Io)?;
        let p = entry.path();
        if p.is_dir() {
            walk(&p, out)?;
        } else if p.extension().and_then(|s| s.to_str()) == Some("js") {
            out.push(p);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn applies_single_patch_inserts_at_start_of_match() {
        let tmp = tempfile::tempdir().unwrap();
        write(
            tmp.path(),
            "a.js",
            "x; log.info(\"updateMessagesMeRead\", t);",
        );
        let rules = vec![PatchRule {
            id: "r1".into(),
            regex: r#"\w+\.info\("updateMessagesMeRead""#.into(),
            payload: "/*PATCHED*/".into(),
            required_hits: 1,
        }];
        let report = apply_patches(tmp.path(), &rules).unwrap();
        assert_eq!(report.patches_hit, 1);
        assert_eq!(report.per_patch["r1"], 1);
        let content = fs::read_to_string(tmp.path().join("a.js")).unwrap();
        assert!(content.contains("/*PATCHED*/log.info"), "got: {content}");
    }

    #[test]
    fn missing_required_hits_returns_patch_no_hit() {
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "a.js", "x = 1;");
        let rules = vec![PatchRule {
            id: "r1".into(),
            regex: r#"updateMessagesMeRead"#.into(),
            payload: "/*P*/".into(),
            required_hits: 1,
        }];
        let err = apply_patches(tmp.path(), &rules).unwrap_err();
        match err {
            AppError::PatchNoHit { patch_id } => assert_eq!(patch_id, "r1"),
            other => panic!("expected PatchNoHit, got {other:?}"),
        }
    }

    #[test]
    fn matches_full_member_chain_with_dot_in_charset() {
        // 飞书 minified 后写成 `l.Ay.info("X"`,旧 regex `\w+\.info\(...)` 只命中
        // `Ay.info(...)`,注入会破坏成 `l.<INJECT>Ay.info(...)`,运行时把 INJECT
        // 当成 `l` 的成员访问,语义错。新 regex `[\w.]+\.info\(...)` 必须命中
        // 整个 `l.Ay.info(...)`,注入到 `l` 前。
        let tmp = tempfile::tempdir().unwrap();
        write(tmp.path(), "a.js", "x; l.Ay.info(\"updateMessagesMeRead\", t);");
        let rules = vec![PatchRule {
            id: "r1".into(),
            regex: r#"[\w.]+\.info\("updateMessagesMeRead""#.into(),
            payload: "INJ;".into(),
            required_hits: 1,
        }];
        apply_patches(tmp.path(), &rules).unwrap();
        let content = fs::read_to_string(tmp.path().join("a.js")).unwrap();
        assert!(
            content.contains("INJ;l.Ay.info(\"updateMessagesMeRead\""),
            "got: {content}"
        );
        assert!(!content.contains("l.INJ;Ay"), "must not break member chain");
    }

    #[test]
    fn reverse_order_keeps_offsets_correct_for_multiple_hits_in_one_file() {
        let tmp = tempfile::tempdir().unwrap();
        // 同一锚点在一个文件里出现两次
        write(
            tmp.path(),
            "a.js",
            "log.info(\"updateMessagesMeRead\", a); log.info(\"updateMessagesMeRead\", b);",
        );
        let rules = vec![PatchRule {
            id: "r1".into(),
            regex: r#"log\.info\("updateMessagesMeRead""#.into(),
            payload: "X".into(),
            required_hits: 2,
        }];
        let report = apply_patches(tmp.path(), &rules).unwrap();
        assert_eq!(report.per_patch["r1"], 2);
        let content = fs::read_to_string(tmp.path().join("a.js")).unwrap();
        // 两处都插入,且不互相错位
        let count = content
            .matches("Xlog.info(\"updateMessagesMeRead\"")
            .count();
        assert_eq!(count, 2, "got: {content}");
    }
}
