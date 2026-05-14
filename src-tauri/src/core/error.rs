use std::path::PathBuf;
use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("未在以下路径找到飞书安装:{0:?}")]
    FeishuNotFound(Vec<PathBuf>),

    #[error("飞书正在运行,请先退出飞书")]
    FeishuRunning,

    #[error("文件被占用,无法写入:{0}")]
    AsarLocked(PathBuf),

    #[error("asar 文件解析失败:{0}")]
    AsarMalformed(String),

    #[error("备份文件已存在:{0}")]
    BackupExists(PathBuf),

    #[error("找不到备份文件:{0}")]
    BackupMissing(PathBuf),

    #[error("补丁规则 {patch_id} 没有命中任何锚点")]
    PatchNoHit { patch_id: String },

    #[error("补丁包需要 app ≥ {required},当前 {current}")]
    PatchVersionIncompatible { required: String, current: String },

    #[error("下载远程补丁失败:{0}")]
    RemoteFetchFailed(String),

    #[error("远程补丁签名校验失败")]
    RemoteSignatureInvalid,

    #[error("远程补丁版本不能回退")]
    RemoteVersionRegression,

    #[error("没有写权限:{0}")]
    PermissionDenied(PathBuf),

    #[error("应用更新失败:{0}")]
    UpdaterFailed(String),

    #[error("IO 错误:{0}")]
    Io(#[from] std::io::Error),

    #[error("内部错误:{0}")]
    Internal(String),
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            kind: &'a str,
            message: String,
        }
        let kind = match self {
            AppError::FeishuNotFound(_) => "FeishuNotFound",
            AppError::FeishuRunning => "FeishuRunning",
            AppError::AsarLocked(_) => "AsarLocked",
            AppError::AsarMalformed(_) => "AsarMalformed",
            AppError::BackupExists(_) => "BackupExists",
            AppError::BackupMissing(_) => "BackupMissing",
            AppError::PatchNoHit { .. } => "PatchNoHit",
            AppError::PatchVersionIncompatible { .. } => "PatchVersionIncompatible",
            AppError::RemoteFetchFailed(_) => "RemoteFetchFailed",
            AppError::RemoteSignatureInvalid => "RemoteSignatureInvalid",
            AppError::RemoteVersionRegression => "RemoteVersionRegression",
            AppError::PermissionDenied(_) => "PermissionDenied",
            AppError::UpdaterFailed(_) => "UpdaterFailed",
            AppError::Io(_) => "Io",
            AppError::Internal(_) => "Internal",
        };
        Wire { kind, message: self.to_string() }.serialize(s)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
