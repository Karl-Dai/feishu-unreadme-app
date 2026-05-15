#!/usr/bin/env node
// 从 CHANGELOG.md 提取指定版本段(含其下所有内容,不含版本标题行本身)。
// 用法:node scripts/extract-changelog-section.mjs v0.1.1
//      node scripts/extract-changelog-section.mjs 0.1.1
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const raw = process.argv[2];
if (!raw) {
  console.error('usage: extract-changelog-section.mjs <version|tag>');
  process.exit(1);
}
const version = raw.replace(/^v/, '');
const escaped = version.replace(/\./g, '\\.');

const text = readFileSync(resolve(process.cwd(), 'CHANGELOG.md'), 'utf8');
const re = new RegExp(
  `(?:^|\\n)## \\[${escaped}\\][^\\n]*\\n([\\s\\S]*?)(?=\\n## \\[|$)`,
);
const m = re.exec(text);
if (!m) {
  console.error(`No section found for version ${version} in CHANGELOG.md`);
  process.exit(2);
}
process.stdout.write(m[1].trim() + '\n');
