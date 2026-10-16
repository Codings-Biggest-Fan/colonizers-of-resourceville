[CmdletBinding()]
param()

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot

try {
    Write-Host "Repository: $repoRoot"
    Write-Host 'Read-only prerequisite check. No software will be installed.'
    $missing = [System.Collections.Generic.List[string]]::new()
    foreach ($tool in @('rustc', 'cargo', 'rustup', 'git')) {
        $command = Get-Command $tool -ErrorAction SilentlyContinue
        if ($null -eq $command) {
            $candidate = Join-Path $env:USERPROFILE ".cargo\bin\$tool.exe"
            if (Test-Path -LiteralPath $candidate -PathType Leaf) {
                $command = Get-Command $candidate -ErrorAction Stop
            }
        }
        if ($null -eq $command) {
            $missing.Add($tool)
            Write-Warning "$tool was not found on PATH or in the default Cargo bin directory."
        } else {
            & $command.Source --version
            if ($LASTEXITCODE -ne 0) {
                throw "$tool --version failed with exit code $LASTEXITCODE."
            }
        }
    }

    if ($env:OS -eq 'Windows_NT') {
        $programFilesX86 = [Environment]::GetFolderPath('ProgramFilesX86')
        $vswhere = if ($programFilesX86) {
            Join-Path $programFilesX86 'Microsoft Visual Studio\Installer\vswhere.exe'
        } else { $null }
        $msvc = Get-Command cl.exe -ErrorAction SilentlyContinue
        $buildTools = @()
        if ($vswhere -and (Test-Path -LiteralPath $vswhere)) {
            $buildTools = @(& $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath)
            if ($LASTEXITCODE -ne 0) { throw 'Visual Studio prerequisite query failed.' }
        }
        if (($null -eq $msvc) -and $buildTools.Count -eq 0) {
            $missing.Add('MSVC build tools')
            Write-Warning 'Microsoft C++ build tools were not detected. Verify the Desktop development with C++ workload and Windows SDK before a Rust MSVC build.'
        }
        $sdkRoot = Join-Path $programFilesX86 'Windows Kits/10/Lib'
        $sdkLibraries = @(Get-ChildItem -LiteralPath $sdkRoot -Filter dbghelp.lib -Recurse -File -ErrorAction SilentlyContinue | Where-Object FullName -Match '[\\/]um[\\/]x64[\\/]dbghelp\.lib$')
        if ($sdkLibraries.Count -eq 0) {
            $missing.Add('Windows SDK x64 libraries')
            Write-Warning 'Windows SDK dbghelp.lib was not found. Install the Windows SDK explicitly and finish installation before building.'
        }
    } else {
        $missing.Add('Windows release environment')
        Write-Warning 'Windows is the supported release target; this is not a Windows environment.'
    }

    if (-not (Test-Path -LiteralPath (Join-Path $repoRoot 'Cargo.toml'))) {
        Write-Host 'Cargo workspace is not scaffolded yet; application checks are pending milestone 01.'
    }
    if ($missing.Count -gt 0) {
        Write-Error ("Prerequisites missing or unverified: " + ($missing -join ', ') + '. See https://rust-lang.org/tools/install/ and the Microsoft C++ Build Tools instructions. Install explicitly, then rerun this helper.') -ErrorAction Continue
        exit 1
    }
    Write-Host 'Prerequisite detection passed. This is not a build or native-runtime verification.'
    exit 0
} catch {
    Write-Error $_ -ErrorAction Continue
    exit 1
}
