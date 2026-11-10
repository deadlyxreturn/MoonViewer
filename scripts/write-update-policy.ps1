<# Writes the Moon Viewer release policy from the checkout's defaults. #>
param(
    [Parameter(Mandatory)][ValidatePattern('^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$')][string]$Tag,
    [string]$Out = 'moon-viewer-update-policy.json'
)
$ErrorActionPreference = 'Stop'
. "$PSScriptRoot/tools/update-policy.ps1"
$repo = Split-Path $PSScriptRoot -Parent
# An imported Prototype tag must not dictate Moon Viewer's settings policy.
$policy = Get-UpdatePolicy -RepoPath $repo -Tag $Tag
Assert-UpdatePolicy $policy $Tag.Substring(1)
$json = $policy | ConvertTo-Json -Depth 4
$path = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($Out)
[IO.File]::WriteAllText($path, $json, [Text.UTF8Encoding]::new($false))
