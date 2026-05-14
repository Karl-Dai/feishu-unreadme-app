use serde::{Deserialize, Serialize};

use crate::core::error::{AppError, Result};
use crate::core::patcher::PatchRule;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatchSet {
    pub version: String,
    pub min_app_version: String,
    pub patches: Vec<PatchRule>,
}

const BUILTIN: &str = include_str!("../../resources/patches.builtin.json");

pub fn load_builtin() -> Result<PatchSet> {
    serde_json::from_str(BUILTIN)
        .map_err(|e| AppError::Internal(format!("内置 patches 解析失败:{e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_loads_and_has_two_patches() {
        let set = load_builtin().unwrap();
        assert!(!set.version.is_empty());
        assert_eq!(set.patches.len(), 2);
        let ids: Vec<&str> = set.patches.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"block-updateMessagesMeRead"));
        assert!(ids.contains(&"allow-on-send-success"));
    }

    #[test]
    fn min_app_version_matches_current_for_v010() {
        let set = load_builtin().unwrap();
        assert_eq!(set.min_app_version, "0.1.0");
    }
}
