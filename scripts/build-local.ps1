param([switch]$Dev, [switch]$TestsOnly)
$ErrorActionPreference = 'Stop'
Set-Location (Split-Path $PSScriptRoot -Parent)
# Portable toolchain installed during this project's initial build.
# Use normal system Rust when the portable installation is unavailable.
if (Test-Path -LiteralPath 'D:\VEXORYN-build\cargo\bin\cargo.exe') {
  $env:CARGO_HOME = 'D:\VEXORYN-build\cargo'
  $env:RUSTUP_HOME = 'D:\VEXORYN-build\rustup'
  $env:CARGO_TARGET_DIR = 'D:\VEXORYN-build\target'
  $env:TEMP = 'D:\VEXORYN-build'
  $env:TMP = 'D:\VEXORYN-build'
  $env:Path = 'D:\VEXORYN-build\cargo\bin;' + $env:Path
}
function Check-Exit { if ($LASTEXITCODE -ne 0) { throw "Command failed: $LASTEXITCODE" } }
if ($Dev) { npm.cmd run tauri -- dev; Check-Exit; exit }
npm.cmd test
Check-Exit
node --test website/site.test.mjs
Check-Exit
npm.cmd run build
Check-Exit
cargo test --locked --manifest-path src-tauri/Cargo.toml
Check-Exit
node scripts/licenses.mjs
Check-Exit
if (!$TestsOnly) { npm.cmd run tauri -- build --bundles nsis,msi; Check-Exit }
