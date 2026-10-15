[CmdletBinding(DefaultParameterSetName = 'All')]
param(
    [Parameter(ParameterSetName = 'Single', Mandatory = $true)]
    [ValidateRange(0, 20)]
    [int] $Number,
    [Parameter(ParameterSetName = 'All')]
    [switch] $All,
    [switch] $RequireComplete
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$repoRoot = [System.IO.Path]::GetFullPath((Split-Path -Parent $PSScriptRoot))
$errorsFound = [System.Collections.Generic.List[string]]::new()

function Assert-Record {
    param([string] $RelativePath)
    $absolutePath = Join-Path $repoRoot $RelativePath
    if (-not (Test-Path -LiteralPath $absolutePath -PathType Leaf)) {
        $errorsFound.Add("Missing record: $RelativePath")
    } elseif ((Get-Item -LiteralPath $absolutePath).Length -eq 0) {
        $errorsFound.Add("Empty record: $RelativePath")
    }
}

try {
    $requiredRecords = @(
        'AGENTS.md', 'README.md', 'ROADMAP.md', 'CHANGELOG.md',
        'docs/implementation-plan.md', 'docs/architecture.md', 'docs/rules.md',
        'docs/pregame-options.md', 'docs/compatibility.md', 'docs/development.md',
        'docs/deployment.md', 'docs/building.md', 'docs/github-workflow.md',
        'docs/game-instructions/README.md', 'milestones/README.md', 'photos/README.md',
        'scripts/README.md', 'scripts/bootstrap.ps1', 'scripts/milestone.ps1', 'scripts/wiki.ps1',
        'assets/ATTRIBUTION.md', 'supabase/migrations/README.md'
    )
    foreach ($record in $requiredRecords) { Assert-Record $record }

    $selectedNumbers = if ($PSCmdlet.ParameterSetName -eq 'Single') { @($Number) } else { @(0..20) }
    foreach ($milestoneNumber in $selectedNumbers) {
        $slug = 'milestone-{0:D2}' -f $milestoneNumber
        foreach ($record in @("milestones/$slug/plan.md", "milestones/$slug/progress.md", "milestones/$slug/reports/README.md", "photos/$slug/index.md")) {
            Assert-Record $record
        }
        if ($RequireComplete) {
            $progressPath = Join-Path $repoRoot "milestones/$slug/progress.md"
            if (Test-Path -LiteralPath $progressPath) {
                $progress = Get-Content -LiteralPath $progressPath -Raw
                if ($progress -notmatch '(?m)^Status:\s*Complete[ \t]*\r?$') {
                    $errorsFound.Add("$slug is not marked Complete.")
                }
            }
            $reportsPath = Join-Path $repoRoot "milestones/$slug/reports"
            $reports = @(Get-ChildItem -LiteralPath $reportsPath -Filter '*.md' -File -ErrorAction SilentlyContinue | Where-Object Name -Match '^\d{3}-.+\.md$')
            if ($reports.Count -eq 0) { $errorsFound.Add("$slug has no numbered implementation report.") }
        }
    }

    # Walk documentation roots rather than build/vendor trees.
    $markdownFiles = [System.Collections.Generic.List[System.IO.FileInfo]]::new()
    foreach ($file in @(Get-ChildItem -LiteralPath $repoRoot -Filter '*.md' -File)) { $markdownFiles.Add($file) }
    foreach ($directory in @('docs', 'milestones', 'photos', 'scripts', 'assets', 'supabase', 'crates')) {
        $directoryPath = Join-Path $repoRoot $directory
        if (Test-Path -LiteralPath $directoryPath) {
            foreach ($file in @(Get-ChildItem -LiteralPath $directoryPath -Filter '*.md' -Recurse -File)) { $markdownFiles.Add($file) }
        }
    }
    $linkCount = 0
    foreach ($file in $markdownFiles) {
        $content = Get-Content -LiteralPath $file.FullName -Raw
        if ($null -eq $content) { continue }
        foreach ($match in [regex]::Matches($content, '!?\[[^\]]*\]\((?<target>[^)]+)\)')) {
            $target = $match.Groups['target'].Value.Trim()
            if ($target.StartsWith('<') -and $target.EndsWith('>')) { $target = $target.Substring(1, $target.Length - 2) }
            if ($target -match '^[A-Za-z][A-Za-z0-9+.-]*:' -or $target.StartsWith('#')) { continue }
            $pathPart = [System.Uri]::UnescapeDataString(($target -split '[?#]', 2)[0])
            if ([string]::IsNullOrWhiteSpace($pathPart)) { continue }
            $linkCount++
            $resolved = [System.IO.Path]::GetFullPath((Join-Path $file.DirectoryName $pathPart))
            if (($resolved -ne $repoRoot) -and (-not $resolved.StartsWith($repoRoot + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase))) {
                $errorsFound.Add("Local link escapes repository in $($file.FullName): $target")
            } elseif (-not (Test-Path -LiteralPath $resolved)) {
                $errorsFound.Add("Broken local link in $($file.FullName): $target")
            }
        }
    }

    if ($errorsFound.Count -gt 0) {
        foreach ($failure in $errorsFound) { Write-Error $failure -ErrorAction Continue }
        Write-Host "$($errorsFound.Count) validation error(s)."
        exit 1
    }
    Write-Host "PASS: $($selectedNumbers.Count) milestone record set(s), $($markdownFiles.Count) Markdown files, and $linkCount local links."
    Write-Host 'This checks record structure and link targets, not heading anchors, evidence truth, runtime behavior, or release readiness.'
    exit 0
} catch {
    Write-Error $_ -ErrorAction Continue
    exit 1
}
