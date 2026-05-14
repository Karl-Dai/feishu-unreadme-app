#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p .secrets
if [ -f .secrets/updater.key ]; then
  echo ".secrets/updater.key 已存在,跳过生成"
  exit 0
fi
# 非交互方式:用 --password 传入密码,避免向 TTY 提示
PASSWORD="${TAURI_KEY_PASSWORD:-feishu-unreadme-dev}"
pnpm tauri signer generate -w .secrets/updater.key --password "$PASSWORD"
echo
echo "Generated keypair. Public key (will be written into tauri.conf.json):"
cat .secrets/updater.key.pub
echo
echo "Private key path: .secrets/updater.key"
echo "Password used (also needs to be set as TAURI_SIGNING_PRIVATE_KEY_PASSWORD in GH Secrets later): $PASSWORD"
