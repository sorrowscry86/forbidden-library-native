#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const root = process.cwd();
const pkgPath = join(root, 'package.json');
const tauriPath = join(root, 'src-tauri', 'tauri.conf.json');
const cargoPath = join(root, 'Cargo.toml');

function usage() {
  console.error('Usage: node scripts/bump-version.mjs <new-version>');
  process.exit(1);
}

const newVersion = process.argv[2];
if (!newVersion) usage();

// Update package.json
const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
pkg.version = newVersion;
writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');

// Update tauri.conf.json
const tauri = JSON.parse(readFileSync(tauriPath, 'utf8'));
if (tauri.package) {
  tauri.package.version = newVersion;
}
writeFileSync(tauriPath, JSON.stringify(tauri, null, 2) + '\n');

// Update Cargo.toml workspace.package.version
let cargo = readFileSync(cargoPath, 'utf8');
cargo = cargo.replace(/version\s*=\s*"[^"]+"/m, `version = "${newVersion}"`);
writeFileSync(cargoPath, cargo);

console.log(`Version bumped to ${newVersion}`);
