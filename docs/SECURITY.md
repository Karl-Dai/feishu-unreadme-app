# 安全模型

## v0.1.0

v0.1.0 不包含远程补丁拉取,所有补丁规则编译进二进制(`include_str!`)。攻击面仅限:

- Tauri updater 自更新链路。私钥存 GitHub Actions secret,公钥编进 binary。任何更新包必须用配套私钥签名才会被客户端接受。

## v0.2 远程补丁威胁模型(规划)

(待 v0.2 实施时补全)

关键约束(预留):

- 远程 `patches.json` 必须有配套 `.minisig`,客户端用与 updater 共用的公钥校验;失败则丢弃。
- 版本号必须单调递增,客户端拒绝回退到更低 `version`。
- 客户端对 `payload` 做简单黑名单审查(纵深防御)。

## 漏洞披露

请通过 GitHub Security Advisories 私下报告。
