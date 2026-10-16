<#! Formatting and lint checks. -Release also builds optimized workspace binaries. #>
[CmdletBinding()]
param([switch] $Release)
try {
    . "$PSScriptRoot/common.ps1"
    Push-Location $RepoRoot
    try {
        Invoke-Cargo @('fmt', '--all', '--', '--check')
        Invoke-Cargo @('clippy', '--workspace', '--all-targets', '--locked', '--', '-D', 'warnings')
        if ($Release) { Invoke-Cargo @('build', '--workspace', '--release', '--locked') }
    } finally { Pop-Location }
} catch { Write-Error $_ -ErrorAction Continue; exit 1 }
