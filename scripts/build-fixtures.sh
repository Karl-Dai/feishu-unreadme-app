#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

FIX="src-tauri/tests/fixtures"
mkdir -p "$FIX"

# 用 pnpm dlx 拉 @electron/asar CLI(无需常驻安装)
ASAR="pnpm dlx @electron/asar"

# happy.asar:两条 spec 锚点各命中一次的最小样本
TMP="$(mktemp -d)"
mkdir -p "$TMP/renderer"
cat > "$TMP/renderer/preload.js" <<'EOF'
"use strict";
var log = require("./logger");
function tryReport(t) {
  log.info("updateMessagesMeRead", t);
}
function sendMessage(req) {
  return req.then(function(res) {
    log.info("MessageService::sendMessage:onSendMessageSuccess:", res);
    return res;
  });
}
module.exports = { tryReport: tryReport, sendMessage: sendMessage };
EOF
cat > "$TMP/renderer/logger.js" <<'EOF'
exports.info = function (tag) { console.log(tag); };
EOF
$ASAR pack "$TMP" "$FIX/happy.asar"
rm -rf "$TMP"

# mismatch.asar:锚点名故意改错
TMP="$(mktemp -d)"
mkdir -p "$TMP/renderer"
cat > "$TMP/renderer/preload.js" <<'EOF'
"use strict";
var log = require("./logger");
function tryReport(t) {
  log.info("NOT_THE_RIGHT_TAG", t);
}
EOF
cat > "$TMP/renderer/logger.js" <<'EOF'
exports.info = function (tag) { console.log(tag); };
EOF
$ASAR pack "$TMP" "$FIX/mismatch.asar"
rm -rf "$TMP"

# multihit.asar:第一个锚点在多个 js 文件出现 3 次
TMP="$(mktemp -d)"
mkdir -p "$TMP/renderer"
for i in a b c; do
cat > "$TMP/renderer/mod_$i.js" <<EOF
var log = require("./logger");
function f$i(t) { log.info("updateMessagesMeRead", t); }
exports.f$i = f$i;
EOF
done
cat > "$TMP/renderer/logger.js" <<'EOF'
exports.info = function (tag) { console.log(tag); };
EOF
cat > "$TMP/renderer/send.js" <<'EOF'
var log = require("./logger");
exports.send = function (res) {
  log.info("MessageService::sendMessage:onSendMessageSuccess:", res);
};
EOF
$ASAR pack "$TMP" "$FIX/multihit.asar"
rm -rf "$TMP"

echo "fixtures 生成完成:"
ls -lh "$FIX"
