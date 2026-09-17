[CmdletBinding()]
param(
    [switch]$SkipNativeBuild
)

$ErrorActionPreference = 'Stop'
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$profile = 'release'
Remove-Item Env:CARGO_TARGET_DIR -ErrorAction SilentlyContinue

if (-not $SkipNativeBuild) {
    cargo build --manifest-path (Join-Path $scriptDir 'Cargo.toml') --release
}

$libDir = Join-Path $scriptDir 'FastLogging/lib'
New-Item -ItemType Directory -Force -Path $libDir | Out-Null
$source = Join-Path $repoRoot "target/$profile/jfastlogging_jni.dll"
$destination = Join-Path $libDir 'jfastlogging.dll'
Copy-Item -Force $source $destination

$pom = Join-Path $scriptDir 'FastLogging/pom.xml'
$mavenArgs = @('-f', $pom, 'clean', 'compile')
& mvn @mavenArgs
if ($LASTEXITCODE -ne 0) {
    throw "Maven compilation failed with exit code $LASTEXITCODE."
}

$pomXml = [xml](Get-Content $pom)
$artifactId = $pomXml.project.artifactId
$version = $pomXml.project.version
$targetDir = Join-Path $scriptDir 'FastLogging/target'
$classesDir = Join-Path $targetDir 'classes'
$fastLoggingDir = Join-Path $scriptDir 'FastLogging'
$jarPath = Join-Path $targetDir "$artifactId-$version-windows.jar"
& jar --create --file $jarPath -C $classesDir . -C $fastLoggingDir lib
if ($LASTEXITCODE -ne 0) {
    throw "JAR creation failed with exit code $LASTEXITCODE."
}
Write-Output "Created $jarPath"
