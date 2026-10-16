<#! Start desktop or liveness-only server. RESOURCEVILLE_BIND controls server address. #>
[CmdletBinding()]
param([ValidateSet('Desktop', 'Server')] [string] $App = 'Desktop', [switch] $Release)
try {
    . "$PSScriptRoot/common.ps1"
    Push-Location $RepoRoot
    try {
        $arguments = @('run', '--locked', '-p', "resourceville-$($App.ToLowerInvariant())")
        if ($Release) { $arguments += '--release' }
        Invoke-Cargo $arguments
    } finally { Pop-Location }
} catch { Write-Error $_ -ErrorAction Continue; exit 1 }
