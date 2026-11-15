# PowerShell runner for host-side Producer-Consumer prototype
$ErrorActionPreference = "Stop"

Set-Location -Path "$PSScriptRoot\rust-prototype"
Write-Host "=== Running Producer-Consumer Research Prototype (Host) ===" -ForegroundColor Cyan
cargo run --release --target x86_64-pc-windows-msvc

Write-Host "`n=== Prototype Execution Complete! Check research/rust-prototype/results.csv ===" -ForegroundColor Green
