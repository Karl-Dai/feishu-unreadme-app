#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, '..');

const target = process.argv[2];
if (!target || !/^\d+\.\d+\.\d+(-[\w.]+)?$/.test(target)) {
  console.error('用法:pnpm bump <semver>,例如 pnpm bump 0.1.0');
  process.exit(1);
}

function bumpPackageJson() {
  const p = path.join(root, 'package.json');
  const j = JSON.parse(fs.readFileSync(p, 'utf8'));
  j.version = target;
  fs.writeFileSync(p, JSON.stringify(j, null, 2) + '\n');
  console.log(`  package.json → ${target}`);
}

function bumpCargoToml() {
  const p = path.join(root, 'src-tauri/Cargo.toml');
  let c = fs.readFileSync(p, 'utf8');
  c = c.replace(/^version\s*=\s*"[^"]+"$/m, `version = "${target}"`);
  fs.writeFileSync(p, c);
  console.log(`  src-tauri/Cargo.toml → ${target}`);
}

function bumpTauriConf() {
  const p = path.join(root, 'src-tauri/tauri.conf.json');
  const j = JSON.parse(fs.readFileSync(p, 'utf8'));
  j.version = target;
  fs.writeFileSync(p, JSON.stringify(j, null, 2) + '\n');
  console.log(`  src-tauri/tauri.conf.json → ${target}`);
}

function bumpChangelog() {
  const p = path.join(root, 'CHANGELOG.md');
  let c = fs.readFileSync(p, 'utf8');
  const today = new Date().toISOString().slice(0, 10);
  const re = new RegExp(`^## \\[${target}\\] - (?:YYYY-MM-DD|\\d{4}-\\d{2}-\\d{2})\\s*$`, 'm');
  if (re.test(c)) {
    c = c.replace(re, `## [${target}] - ${today}`);
  } else {
    c = c.replace(
      /^## \[Unreleased\]\s*\n/m,
      `## [Unreleased]\n\n## [${target}] - ${today}\n`,
    );
  }
  fs.writeFileSync(p, c);
  console.log(`  CHANGELOG.md → ${target} - ${today}`);
}

console.log(`bumping to v${target}`);
bumpPackageJson();
bumpCargoToml();
bumpTauriConf();
bumpChangelog();
console.log('\n下一步:');
console.log(`  git add -A`);
console.log(`  git commit -m "chore: release v${target}"`);
console.log(`  git tag v${target}`);
console.log(`  git push origin main --tags`);
