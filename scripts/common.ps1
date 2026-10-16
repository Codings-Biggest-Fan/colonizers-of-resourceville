# Shared definitions; dot-source from entry points. Never installs tools.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$script:RepoRoot = Split-Path -Parent $PSScriptRoot
function Get-Cargo {
    $tool = Get-Command cargo -ErrorAction SilentlyContinue
    if ($tool) { return $tool.Source }
    $candidate = Join-Path $env:USERPROFILE '.cargo/bin/cargo.exe'
    if (Test-Path -LiteralPath $candidate) { return $candidate }
    throw 'Cargo not found. Install the pinned Rust toolchain explicitly; run scripts/bootstrap.ps1.'
}
function Invoke-Cargo {
    param([string[]] $CargoArguments)
    & (Get-Cargo) @CargoArguments
    if ($LASTEXITCODE -ne 0) { throw "cargo $($CargoArguments -join ' ') failed ($LASTEXITCODE)." }
}
