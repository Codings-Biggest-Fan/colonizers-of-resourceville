[CmdletBinding()]
param([string] $OutputPath = '.local/wiki-export', [string] $PullRequestUrl = '', [switch] $Approved)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
try {
    $repoRoot = [IO.Path]::GetFullPath((Split-Path -Parent $PSScriptRoot))
    $outputFull = [IO.Path]::GetFullPath((Join-Path $repoRoot $OutputPath))
    $localRoot = [IO.Path]::GetFullPath((Join-Path $repoRoot '.local')) + [IO.Path]::DirectorySeparatorChar
    if (-not $outputFull.StartsWith($localRoot, [StringComparison]::OrdinalIgnoreCase)) { throw 'OutputPath must be inside ignored .local; never overwrite source docs.' }
    $sourceCommit = git -C $repoRoot rev-parse HEAD
    if ($LASTEXITCODE -ne 0) { throw 'Cannot identify source commit.' }
    $sourceBranch = git -C $repoRoot branch --show-current
    if ($LASTEXITCODE -ne 0) { throw 'Cannot identify source branch.' }
    if ($Approved -and $sourceBranch -ne 'stable') { throw 'Approved pages require stable after explicit user approval.' }
    $changes = @(git -C $repoRoot status --porcelain --untracked-files=normal)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot inspect source status.' }
    if ($Approved -and $changes.Count -gt 0) { throw 'Commit approved sources before generating approved docs.' }
    if ($PullRequestUrl -and $PullRequestUrl -notmatch '^https://github\.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/\d+$') { throw 'Supply a project PR URL.' }
    $site = 'https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville'
    $wiki = "$site/wiki"
    $nl = [Environment]::NewLine
    $files = [Collections.Generic.List[IO.FileInfo]]::new()
    foreach ($file in @(Get-ChildItem -LiteralPath $repoRoot -Filter '*.md' -File)) { $files.Add($file) }
    foreach ($dir in @('docs','milestones','photos','scripts','assets','supabase','crates')) {
        foreach ($file in @(Get-ChildItem -LiteralPath (Join-Path $repoRoot $dir) -Filter '*.md' -File -Recurse)) { $files.Add($file) }
    }
    $special = @{
        'README.md'='Project-Overview'; 'ROADMAP.md'='Roadmap'; 'AGENTS.md'='Agent-Guide'; 'CHANGELOG.md'='Changelog'
        'docs/building.md'='Building'; 'docs/github-workflow.md'='GitHub-Workflow'; 'docs/game-instructions/README.md'='Game-Instructions'
        'docs/game-instructions/getting-started.md'='Game-Instructions-Getting-Started'
        'docs/game-instructions/turns-and-trading.md'='Game-Instructions-Turns-and-Trading'
        'docs/game-instructions/bots-saves-and-reconnect.md'='Game-Instructions-Bots-Saves-and-Reconnect'
        'docs/game-instructions/variants-and-expansions.md'='Game-Instructions-Variants-and-Expansions'
        'docs/game-instructions/troubleshooting.md'='Game-Instructions-Troubleshooting'
    }
    $map = @{}
    $slugs = [Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
    foreach ($file in $files) {
        $relative = [IO.Path]::GetRelativePath($repoRoot,$file.FullName).Replace('\','/')
        $slug = if ($special.ContainsKey($relative)) { $special[$relative] } else { ($relative -replace '\.md$','') -replace '[^A-Za-z0-9-]','-' }
        if (-not $slugs.Add($slug)) { throw "Duplicate wiki slug: $slug" }
        $map[$file.FullName] = $slug
    }
    $mode = if ($Approved) { 'APPROVED DOCUMENTATION' } else { 'TESTING PREVIEW — NOT APPROVED' }
    $dirty = if ($changes.Count -gt 0) { ' Uncommitted source edits: commit/regenerate before publication.' } else { '' }
    $review = if ($PullRequestUrl) { " [Review PR]($PullRequestUrl)." } else { ' PR URL not supplied.' }
    $banner = "> **$mode** — branch: $sourceBranch; [commit $($sourceCommit.Substring(0,7))]($site/commit/$sourceCommit).$review$dirty"
    $null = New-Item -ItemType Directory -Path $outputFull -Force
    foreach ($file in $files) {
        $content = Get-Content -LiteralPath $file.FullName -Raw
        $body = [regex]::Replace($content, '(?<prefix>!?\[[^\]]*\]\()(?<target>[^)]+)\)', {
            param($match)
            $target = $match.Groups['target'].Value.Trim()
            if ($target -match '^[A-Za-z][A-Za-z0-9+.-]*:' -or $target.StartsWith('#')) { return $match.Value }
            $parts = $target -split '#',2
            $resolved = [IO.Path]::GetFullPath((Join-Path $file.DirectoryName ([Uri]::UnescapeDataString($parts[0]))))
            $anchor = if ($parts.Count -gt 1) { '#' + $parts[1] } else { '' }
            if (Test-Path -LiteralPath $resolved -PathType Container) { $resolved = Join-Path $resolved 'README.md' }
            if ($map.ContainsKey($resolved)) {
                $destination = "$wiki/$($map[$resolved])$anchor"
            } else {
                $relative = [IO.Path]::GetRelativePath($repoRoot,$resolved).Replace('\','/')
                if ($relative.StartsWith('../')) { throw "Source link escapes repository: $target" }
                $encoded = (($relative -split '/') | ForEach-Object { [Uri]::EscapeDataString($_) }) -join '/'
                $destination = if ($match.Groups['prefix'].Value.StartsWith('!')) { "https://raw.githubusercontent.com/Codings-Biggest-Fan/colonizers-of-resourceville/$sourceCommit/$encoded" } else { "$site/blob/$sourceCommit/$encoded$anchor" }
            }
            return $match.Groups['prefix'].Value + $destination + ')'
        })
        $relative = [IO.Path]::GetRelativePath($repoRoot,$file.FullName).Replace('\','/')
        $page = $banner + $nl + $nl + "[Reviewed source]($site/blob/$sourceCommit/$relative)" + $nl + $nl + $body
        [IO.File]::WriteAllText((Join-Path $outputFull "$($map[$file.FullName]).md"), $page, [Text.UTF8Encoding]::new($false))
    }
    $currentStatus = Get-Content -LiteralPath (Join-Path $repoRoot 'README.md') | Where-Object { $_.StartsWith('**Current status:**') } | Select-Object -First 1
    $wikiHomePage = @"
# The Colonizers of Resourceville

$banner

$currentStatus

- [Current roadmap]($wiki/Roadmap)
- [Building on Windows]($wiki/Building)
- [Game Instructions]($wiki/Game-Instructions)
- [Branches, PR approval, and releases]($wiki/GitHub-Workflow)
- [Agent rules and Polish mode]($wiki/Agent-Guide)
- [Source repository]($site)

## Game Instructions

- [Getting started]($wiki/Game-Instructions-Getting-Started)
- [Turns and trading]($wiki/Game-Instructions-Turns-and-Trading)
- [Bots, saves, and reconnect]($wiki/Game-Instructions-Bots-Saves-and-Reconnect)
- [Variants and expansions]($wiki/Game-Instructions-Variants-and-Expansions)
- [Troubleshooting]($wiki/Game-Instructions-Troubleshooting)

## Milestone records

[Initial setup plan]($wiki/milestones-milestone-00-plan) · [Initial setup progress]($wiki/milestones-milestone-00-progress). The roadmap links all implementation milestones.

Wiki pages are generated from PR-reviewed source. Preview status is not milestone approval or release evidence.
"@
    $sidebar = @"
## Project docs

- [Home]($wiki)
- [Roadmap]($wiki/Roadmap)
- [Building]($wiki/Building)
- [GitHub workflow]($wiki/GitHub-Workflow)
- [Agent guide]($wiki/Agent-Guide)

## Game Instructions

- [Overview]($wiki/Game-Instructions)
- [Getting started]($wiki/Game-Instructions-Getting-Started)
- [Turns and trading]($wiki/Game-Instructions-Turns-and-Trading)
- [Bots and saves]($wiki/Game-Instructions-Bots-Saves-and-Reconnect)
- [Variants]($wiki/Game-Instructions-Variants-and-Expansions)
- [Troubleshooting]($wiki/Game-Instructions-Troubleshooting)

## Milestones

"@
    foreach ($n in 0..20) {
        $nn = '{0:D2}' -f $n
        $sidebar += "- [M$nn plan]($wiki/milestones-milestone-$nn-plan) · [progress]($wiki/milestones-milestone-$nn-progress)" + $nl
    }
    foreach ($entry in @{ 'Home.md'=$wikiHomePage; '_Sidebar.md'=$sidebar; '_Footer.md'=$banner }.GetEnumerator()) {
        [IO.File]::WriteAllText((Join-Path $outputFull $entry.Key), $entry.Value, [Text.UTF8Encoding]::new($false))
    }
    Write-Host "Generated $($files.Count + 3) wiki files in $outputFull. Mode: $mode."
    Write-Host 'No Git commits, pushes, merges, or releases performed.'
    exit 0
} catch { Write-Error $_ -ErrorAction Continue; exit 1 }
