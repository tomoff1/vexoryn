$ErrorActionPreference = 'Stop'
$taskRoot = Split-Path $PSScriptRoot -Parent
Set-Location $taskRoot
$taskStage = Join-Path $taskRoot 'artifacts/source/VEXORYN'
New-Item -ItemType Directory -Force $taskStage | Out-Null
$taskFiles = @('README.md','LICENSE','THIRD_PARTY.md','THIRD_PARTY_LICENSES.txt','.gitignore','package.json','package-lock.json','tsconfig.json','vite.config.ts','index.html')
foreach ($taskFile in $taskFiles) { Copy-Item -LiteralPath (Join-Path $taskRoot $taskFile) -Destination $taskStage -Force }
foreach ($taskDir in @('src','docs','website','tests','scripts','.github')) { Copy-Item -LiteralPath (Join-Path $taskRoot $taskDir) -Destination $taskStage -Recurse -Force }
$taskRust = Join-Path $taskStage 'src-tauri'
New-Item -ItemType Directory -Force $taskRust | Out-Null
foreach ($taskFile in @('Cargo.toml','Cargo.lock','build.rs','tauri.conf.json','API.md')) { Copy-Item -LiteralPath (Join-Path $taskRoot "src-tauri/$taskFile") -Destination $taskRust -Force }
foreach ($taskDir in @('src','icons','capabilities')) { Copy-Item -LiteralPath (Join-Path $taskRoot "src-tauri/$taskDir") -Destination $taskRust -Recurse -Force }
$taskArchive = Join-Path $taskRoot 'artifacts/VEXORYN-0.1.0-source.zip'
Compress-Archive -Path $taskStage -DestinationPath $taskArchive -Force
Write-Output $taskArchive
