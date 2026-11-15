# Runs the same checks as CI on your own Windows machine.
# Usage (PowerShell, in the repo folder):  .\scripts\check.ps1
# Needs Rust (https://rustup.rs) and git.
$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)

function Step($name, $cmd) {
    Write-Host "`n== $name ==" -ForegroundColor Cyan
    & $cmd
    if ($LASTEXITCODE -ne 0) {
        Write-Host "FAILED: $name" -ForegroundColor Red
        exit 1
    }
}

Step "Format"  { cargo fmt --all --check }
Step "Lint"    { cargo clippy --all-targets -- -D warnings }
Step "Tests"   { cargo test --all --locked }
Step "Build"   { cargo build --release --locked -p agentdisk }
# Cargo puts the binary under CARGO_TARGET_DIR when it is set (a relative path is
# relative to the repo root, where this script runs).
Write-Host "`n== Smoke ==" -ForegroundColor Cyan
$targetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "target" }
$bin = Join-Path (Join-Path $targetDir "release") "agentdisk.exe"
if (-not (Test-Path -LiteralPath $bin -PathType Leaf)) {
    Write-Host "error: smoke test binary not found at $bin" -ForegroundColor Red
    exit 1
}

# Runs one smoke command and stops the script when it exits non-zero.
function Smoke([string[]]$SmokeArgs, [switch]$Quiet) {
    if ($Quiet) { & $bin @SmokeArgs | Out-Null } else { & $bin @SmokeArgs }
    if ($LASTEXITCODE -ne 0) {
        Write-Host "FAILED: Smoke (agentdisk $($SmokeArgs -join ' ') exited with $LASTEXITCODE)" -ForegroundColor Red
        exit 1
    }
}

Smoke "--version"
Smoke "adapters" -Quiet
Smoke "size", "." -Quiet

Write-Host "`nAll checks passed. Binary: $bin" -ForegroundColor Green
