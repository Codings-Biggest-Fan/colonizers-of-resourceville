<#! Capture only the game's rendered frame through eframe, never the desktop.
Starts a fresh native instance which exits after capture. Zoom is egui zoom,
not proof of changing the operating system's display scaling. #>
[CmdletBinding()]
param(
    [ValidateRange(1, 20)] [int] $Milestone = 1,
    [Parameter(Mandatory)] [ValidatePattern('^\d{3}-[a-z0-9-]+\.png$')] [string] $Name,
    [ValidateRange(480, 2400)] [int] $Width = 1000,
    [ValidateRange(420, 1600)] [int] $Height = 680,
    [ValidateRange(0.75, 2)] [double] $Zoom = 1,
    [ValidateSet('layout','setup','production','rejection','contrast','zoomed-out')] [string] $Scenario = 'layout'
)
try {
    . "$PSScriptRoot/common.ps1"
    Push-Location $RepoRoot
    try {
        $destination = Join-Path $RepoRoot ('photos/milestone-{0:D2}/{1}' -f $Milestone, $Name)
        if (Test-Path -LiteralPath $destination) { throw 'Capture exists; preserve prior evidence and choose a new sequence.' }
        Invoke-Cargo @('build', '--locked', '-p', 'resourceville-desktop', '--features', 'capture')
        $names = @('RESOURCEVILLE_CAPTURE_TO','RESOURCEVILLE_CAPTURE_WIDTH','RESOURCEVILLE_CAPTURE_HEIGHT','RESOURCEVILLE_CAPTURE_SCALE','RESOURCEVILLE_CAPTURE_SCENARIO')
        $previous = @{}
        foreach ($key in $names) { $previous[$key] = [Environment]::GetEnvironmentVariable($key) }
        try {
            $env:RESOURCEVILLE_CAPTURE_TO = $destination
            $env:RESOURCEVILLE_CAPTURE_SCENARIO = $Scenario
            $env:RESOURCEVILLE_CAPTURE_WIDTH = "$Width"
            $env:RESOURCEVILLE_CAPTURE_HEIGHT = "$Height"
            $env:RESOURCEVILLE_CAPTURE_SCALE = $Zoom.ToString([Globalization.CultureInfo]::InvariantCulture)
            $exe = if ($IsWindows) { 'target/debug/resourceville-desktop.exe' } else { 'target/debug/resourceville-desktop' }
            & $exe
            if ($LASTEXITCODE -ne 0) { throw "Native capture process failed ($LASTEXITCODE)." }
            if (-not (Test-Path -LiteralPath $destination)) { throw 'Native renderer did not produce the requested capture.' }
            Write-Host "Captured $destination. Inspect it and record actual dimensions/scaling in the photo index."
        } finally {
            foreach ($key in $names) { [Environment]::SetEnvironmentVariable($key, $previous[$key]) }
        }
    } finally { Pop-Location }
} catch { Write-Error $_ -ErrorAction Continue; exit 1 }


