// Collect installed dependency notices without copying local absolute paths.
import fs from 'node:fs';
import path from 'node:path';
import { execFileSync } from 'node:child_process';
const root = process.cwd();
const notices = ['VEXORYN — dependency license notices\nGenerated from the locked dependency graph.\n'];
function collect(name, version, license, dir) {
  notices.push(`\n${'='.repeat(72)}\n${name} ${version}\nLicense expression: ${license || 'See upstream license'}\n`);
  for (const entry of fs.readdirSync(dir, { withFileTypes: true }).filter(e => e.isFile() && /^(licen[sc]e|copying|notice)([._-].*)?$/i.test(e.name))) {
    const content = fs.readFileSync(path.join(dir, entry.name), 'utf8');
    notices.push(`\n--- ${entry.name} ---\n${content}\n`);
  }
}
const metadata = JSON.parse(execFileSync('cargo', ['metadata', '--locked', '--offline', '--format-version', '1', '--filter-platform', 'x86_64-pc-windows-msvc', '--manifest-path', 'src-tauri/Cargo.toml'], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }));
for (const pkg of metadata.packages.filter(p => p.source).sort((a, b) => a.name.localeCompare(b.name))) collect(pkg.name, pkg.version, pkg.license, path.dirname(pkg.manifest_path));
const lock = JSON.parse(fs.readFileSync('package-lock.json', 'utf8'));
for (const [location, pkg] of Object.entries(lock.packages).filter(([key]) => key.startsWith('node_modules/')).sort(([a], [b]) => a.localeCompare(b))) {
  const dir = path.join(root, location);
  if (!fs.existsSync(dir)) continue;
  collect(location.replace(/^node_modules\//, ''), pkg.version, pkg.license, dir);
}
fs.writeFileSync('THIRD_PARTY_LICENSES.txt', notices.join(''), 'utf8');
console.log(`License notices generated for ${metadata.packages.filter(p => p.source).length} Rust dependencies and installed npm packages.`);
