<#! Run all tests, or one implemented crate suite. No gameplay suites exist yet. #>
[CmdletBinding()]
param([ValidateSet('All', 'Rules', 'Bots', 'Server')] [string] $Suite = 'All')
try {
    . "$PSScriptRoot/common.ps1"
    Push-Location $RepoRoot
    try {
        $arguments = @('test', '--locked')
        if ($Suite -eq 'All') { $arguments += '--workspace' }
        else { $arguments += @('-p', "resourceville-$($Suite.ToLowerInvariant())") }
        Invoke-Cargo $arguments
    } finally { Pop-Location }
} catch { Write-Error $_ -ErrorAction Continue; exit 1 }
