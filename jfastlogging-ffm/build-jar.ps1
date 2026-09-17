[CmdletBinding()]
param(
    [switch]$SkipNativeBuild
)

$ErrorActionPreference = 'Stop'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$profile = 'release'

if (-not $SkipNativeBuild) {
    cargo build --manifest-path (Join-Path $scriptDir 'Cargo.toml') --release
}

$libDir = Join-Path $scriptDir 'FastLogging/lib'
New-Item -ItemType Directory -Force -Path $libDir | Out-Null
$source = Join-Path $repoRoot "target/$profile/jfastlogging_ffm.dll"
$destination = Join-Path $libDir 'jfastlogging.dll'
Copy-Item -Force $source $destination

mvn -f (Join-Path $scriptDir 'FastLogging/pom.xml') clean package
$jar = Join-Path $scriptDir 'FastLogging/target/FastLogging-0.9.0-ffm.jar'
Write-Output "Created $jar"
