<#! Package current-host release binaries. CI calls this on all six OS/architecture runners.
No signing, installer, release publication, or tool installation occurs. #>
[CmdletBinding()]
param([switch] $SkipBuild)
try {
    . "$PSScriptRoot/common.ps1"
    Push-Location $RepoRoot
    try {
        $changes = @(git status --porcelain --untracked-files=normal)
        if ($LASTEXITCODE -ne 0) { throw 'Cannot inspect source status.' }
        if ($changes.Count -gt 0) { throw 'Commit source and evidence changes before packaging so BUILD.txt identifies the actual source revision.' }
        $rustInfo = & (Get-Cargo) -vV
        if ($LASTEXITCODE -ne 0) { throw 'Cannot inspect Cargo host.' }
        $hostTriple = ($rustInfo | Select-String '^host: (.+)$').Matches.Groups[1].Value
        if (-not $hostTriple) { throw 'Cannot determine Rust host triple.' }
        if (-not $SkipBuild) { Invoke-Cargo @('build', '--workspace', '--release', '--locked') }
        $revision = git rev-parse HEAD
        if ($LASTEXITCODE -ne 0) { throw 'Cannot identify source revision.' }
        $bundle = Join-Path $RepoRoot "dist/resourceville-$hostTriple-$($revision.Substring(0, 7))"
        if (Test-Path -LiteralPath $bundle) { throw "Bundle already exists: $bundle. Choose a fresh checkout/output; existing artifacts are preserved." }
        $null = New-Item -ItemType Directory -Path $bundle
        $suffix = if ($IsWindows) { '.exe' } else { '' }
        foreach ($binary in @('resourceville-desktop', 'resourceville-server')) {
            Copy-Item -LiteralPath "target/release/$binary$suffix" -Destination $bundle
        }
        Copy-Item -LiteralPath README.md, Cargo.lock, assets/ATTRIBUTION.md -Destination $bundle
        # Preserve dependency notices, including bundled font licenses.
        $metadataText = & (Get-Cargo) metadata --locked --format-version 1
        if ($LASTEXITCODE -ne 0) { throw 'Cannot collect dependency license metadata.' }
        $metadata = ($metadataText -join "`n") | ConvertFrom-Json
        $licenseRoot = Join-Path $bundle 'third-party'
        $null = New-Item -ItemType Directory -Path $licenseRoot
        $inventory = @()
        foreach ($dependency in $metadata.packages | Where-Object { $null -ne $_.source }) {
            $inventory += "$($dependency.name) $($dependency.version): $($dependency.license)"
            $sourceRoot = Split-Path -Parent $dependency.manifest_path
            foreach ($notice in Get-ChildItem -LiteralPath $sourceRoot -Recurse -File | Where-Object { $_.Name -match '^(LICENSE|LICENCE|COPYING|COPYRIGHT|NOTICE|OFL|UFL)([._-]|$)' }) {
                $relative = [IO.Path]::GetRelativePath($sourceRoot, $notice.FullName)
                $noticeTarget = Join-Path $licenseRoot "$($dependency.name)-$($dependency.version)/$relative"
                $null = New-Item -ItemType Directory -Path (Split-Path -Parent $noticeTarget) -Force
                Copy-Item -LiteralPath $notice.FullName -Destination $noticeTarget
                # ZIP cannot represent the 1970 timestamps used by some registry archives.
                (Get-Item -LiteralPath $noticeTarget).LastWriteTime = [datetime]'2000-01-01'
            }
        }
        $inventory | Set-Content (Join-Path $licenseRoot 'DEPENDENCIES.txt')
        "Source: $revision`nHost: $hostTriple`nUnsigned foundation preview; no playable games. Nonlocal runtime is unverified." | Set-Content (Join-Path $bundle 'BUILD.txt')
        Get-ChildItem -LiteralPath $bundle -File -Recurse | Get-FileHash -Algorithm SHA256 | ForEach-Object { "$($_.Hash.ToLowerInvariant())  $([IO.Path]::GetRelativePath($bundle, $_.Path).Replace('\','/'))" } | Set-Content (Join-Path $bundle 'SHA256SUMS.txt')
        $archive = "$bundle.zip"
        Compress-Archive -LiteralPath $bundle -DestinationPath $archive
        Get-FileHash -LiteralPath $archive -Algorithm SHA256 | ForEach-Object { "$($_.Hash.ToLowerInvariant())  $([IO.Path]::GetFileName($_.Path))" } | Set-Content "$archive.sha256"
        Write-Host "Packaged $archive"
    } finally { Pop-Location }
} catch { Write-Error $_ -ErrorAction Continue; exit 1 }
