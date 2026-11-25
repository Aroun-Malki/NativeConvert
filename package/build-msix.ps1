[CmdletBinding()]
param(
    [string]$CertificateThumbprint
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory = $true)][string]$Executable,
        [Parameter(Mandatory = $true)][string[]]$Arguments
    )

    & $Executable @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "'$Executable' failed with exit code $LASTEXITCODE."
    }
}

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $PSScriptRoot 'AppxManifest.xml'
$manifest = [xml](Get-Content -LiteralPath $manifestPath -Raw)
$identity = $manifest.SelectSingleNode("/*[local-name()='Package']/*[local-name()='Identity']")
if ($null -eq $identity) {
    throw "The package identity is missing from '$manifestPath'."
}

if ($identity.GetAttribute('ProcessorArchitecture') -ne 'x64') {
    throw 'This packaging script currently supports only x64 packages.'
}

$rustcOutput = & rustc -vV
$rustcExitCode = $LASTEXITCODE
$hostLine = $rustcOutput | Where-Object { $_ -like 'host:*' } | Select-Object -First 1
if ($rustcExitCode -ne 0 -or $hostLine -notmatch 'x86_64-pc-windows-') {
    throw 'An x64 Windows Rust toolchain is required to build this package.'
}

$sdkRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
if (-not (Test-Path -LiteralPath $sdkRoot)) {
    throw "Windows 10 SDK tools were not found at '$sdkRoot'."
}

$makeAppx = Get-ChildItem -LiteralPath $sdkRoot -Filter 'makeappx.exe' -Recurse |
    Where-Object {
        $_.Directory.Name -eq 'x64' -and
        (Test-Path -LiteralPath (Join-Path $_.Directory.FullName 'signtool.exe'))
    } |
    Sort-Object { [version]$_.Directory.Parent.Name } -Descending |
    Select-Object -First 1
if ($null -eq $makeAppx) {
    throw 'Could not find x64 makeappx.exe and signtool.exe in the Windows 10 SDK.'
}
$signTool = Join-Path $makeAppx.Directory.FullName 'signtool.exe'

$applicationManifest = Join-Path $repositoryRoot 'Cargo.toml'
$shellManifest = Join-Path $repositoryRoot 'nativeconvert_shell\Cargo.toml'
Push-Location $repositoryRoot
try {
    Invoke-CheckedCommand -Executable 'cargo' -Arguments @('build', '--release', '--manifest-path', $applicationManifest)
    Invoke-CheckedCommand -Executable 'cargo' -Arguments @('build', '--release', '--manifest-path', $shellManifest)
}
finally {
    Pop-Location
}

$applicationExe = Join-Path $repositoryRoot 'target\release\NativeConvert.exe'
$shellDll = Join-Path $repositoryRoot 'nativeconvert_shell\target\release\nativeconvert_shell.dll'
$icon = Join-Path $repositoryRoot 'assets\logo.ico'
$assetDirectory = Join-Path $PSScriptRoot 'Assets'
$requiredAssets = @('Square44x44Logo.png', 'Square150x150Logo.png', 'StoreLogo.png')
foreach ($requiredFile in @($applicationExe, $shellDll, $icon)) {
    if (-not (Test-Path -LiteralPath $requiredFile -PathType Leaf)) {
        throw "Required package file not found: '$requiredFile'."
    }
}
foreach ($asset in $requiredAssets) {
    $assetPath = Join-Path $assetDirectory $asset
    if (-not (Test-Path -LiteralPath $assetPath -PathType Leaf)) {
        throw "Required package asset not found: '$assetPath'."
    }
}

$outputDirectory = Join-Path $PSScriptRoot 'dist'
New-Item -ItemType Directory -Path $outputDirectory -Force | Out-Null
$packageName = '{0}_{1}_{2}.msix' -f `
    $identity.GetAttribute('Name'),
    $identity.GetAttribute('Version'),
    $identity.GetAttribute('ProcessorArchitecture')
$packagePath = Join-Path $outputDirectory $packageName
$stagingDirectory = Join-Path ([System.IO.Path]::GetTempPath()) `
    ('NativeConvert-msix-' + [guid]::NewGuid().ToString('N'))

try {
    New-Item -ItemType Directory -Path $stagingDirectory | Out-Null
    Copy-Item -LiteralPath $manifestPath -Destination (Join-Path $stagingDirectory 'AppxManifest.xml')
    Copy-Item -LiteralPath $applicationExe -Destination $stagingDirectory
    Copy-Item -LiteralPath $shellDll -Destination $stagingDirectory
    Copy-Item -LiteralPath $icon -Destination $stagingDirectory
    Copy-Item -LiteralPath $assetDirectory -Destination (Join-Path $stagingDirectory 'Assets') -Recurse

    Invoke-CheckedCommand -Executable $makeAppx.FullName -Arguments @(
        'pack', '/d', $stagingDirectory, '/p', $packagePath, '/o'
    )

    if ($CertificateThumbprint) {
        $thumbprint = $CertificateThumbprint.Replace(' ', '')
        $certificate = Get-Item -LiteralPath "Cert:\CurrentUser\My\$thumbprint" -ErrorAction SilentlyContinue
        if ($null -eq $certificate) {
            throw "Signing certificate '$thumbprint' was not found in the current user's Personal certificate store."
        }
        if ($certificate.Subject -ne $identity.GetAttribute('Publisher')) {
            throw "The signing certificate subject '$($certificate.Subject)' must match the package publisher '$($identity.GetAttribute('Publisher'))'."
        }
        if (-not $certificate.HasPrivateKey -or $certificate.NotAfter -le (Get-Date)) {
            throw 'The signing certificate must have an accessible private key and must not be expired.'
        }

        Invoke-CheckedCommand -Executable $signTool -Arguments @(
            'sign', '/fd', 'SHA256', '/s', 'My', '/sha1', $thumbprint, $packagePath
        )
        Export-Certificate -Cert $certificate -FilePath (Join-Path $outputDirectory 'NativeConvertDev.cer') -Force | Out-Null
        Write-Output "Signed MSIX package created: $packagePath"
        Write-Output "Public signing certificate exported: $(Join-Path $outputDirectory 'NativeConvertDev.cer')"
    }
    else {
        Write-Warning 'The MSIX package is unsigned and cannot be installed until it is signed.'
        Write-Output "Unsigned MSIX package created: $packagePath"
    }
}
finally {
    if (Test-Path -LiteralPath $stagingDirectory) {
        Remove-Item -LiteralPath $stagingDirectory -Recurse -Force
    }
}
