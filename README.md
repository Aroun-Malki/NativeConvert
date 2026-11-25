
# NativeConvert

**Convert files. Not websites.**

NativeConvert is an open-source project for native file conversion. It is currently an early-stage Windows prototype.

The current prototype can register a Windows context-menu entry for supported image files and directories.

The conversion engine is not implemented yet, so the app does not convert files at this time.

---

## Why NativeConvert?

File conversion is often unnecessarily complicated.

Users frequently need to:
- Find an online conversion website.
- Upload their files.
- Wait for processing.
- Download the converted files.

NativeConvert aims to eliminate these unnecessary steps by providing a native, local-first alternative.

The project is intended to process files locally without requiring a browser or cloud service.

## Current Status

The Windows prototype can add or remove a NativeConvert entry in the current user's context menu. When launched from that entry, it receives the selected path; conversion is not yet available.

The following features are planned:

- **Context-menu conversion** — Convert files directly from your operating system's context menu.
- **One-Click Conversion** — Convert files without opening a separate application.
- **Batch Processing** — Convert multiple files simultaneously.
- **Local Processing** — Keep files on your machine without uploading them to external servers.
- **Multiple Formats** — Support a wide range of file formats.
- **Conversion Settings** — Customize output quality and conversion options.
- **Cross-Platform** — Designed to support Windows, Linux and macOS.
- **Extensible Architecture** — Modular conversion engines for future format support.

## Platform Support

NativeConvert is designed with a cross-platform architecture.

The initial development and testing target is Windows.

| Platform | Status |
|----------|--------|
| Windows | Initial development |
| Linux | Planned |
| macOS | Planned |

## Supported Formats

NativeConvert is currently in its early development stage.

The initial focus is image conversion.

### Images

| Format | Read | Write |
|--------|------|-------|
| PNG | Planned | Planned |
| JPEG / JPG | Planned | Planned |
| WebP | Planned | Planned |
| HEIC / HEIF | Planned | Planned |
| AVIF | Planned | Planned |
| GIF | Planned | Planned |
| BMP | Planned | Planned |
| TIFF | Planned | Planned |

### Future Categories

- Video
- Audio
- Documents
- Archives

Format availability will depend on the conversion engines and their respective libraries.

## Technology Stack

NativeConvert is built with technologies chosen for performance, reliability and cross-platform compatibility.

- **Rust** — Core application and conversion engine.
- **Windows Shell API** — Native Windows context menu integration.
- **Platform APIs** — Operating system-specific integrations.
- **Open-source libraries** — File decoding and encoding.

## Architecture

NativeConvert follows a modular architecture designed to separate the conversion logic from operating system integrations.

The core conversion engine is intended to remain platform-independent, while each operating system will have its own integration layer.

This approach allows the same conversion logic to be reused across Windows, Linux and macOS.

## Roadmap

### Foundation
- [x] Initialize GitHub repository.
- [x] Set up the Rust development environment.
- [x] Create the initial Rust project.
- [ ] Design the core architecture.

### Image Conversion
- [ ] Implement image format detection.
- [ ] Implement PNG and JPEG conversion.
- [ ] Add WebP support.
- [ ] Add HEIC / HEIF support.
- [ ] Add AVIF support.
- [ ] Implement batch conversion.
- [ ] Add conversion quality settings.

### User Experience
- [ ] Design the conversion workflow.
- [ ] Add a graphical settings interface.
- [ ] Implement conversion progress and error handling.
- [ ] Preserve original files by default.

### Operating System Integration
- [x] Windows context menu registration (basic prototype).
- [ ] Linux file manager integration.
- [ ] macOS Finder integration.

### Future Development
- [ ] Video conversion engine.
- [ ] Audio conversion engine.
- [ ] Document conversion engine.
- [ ] Plugin architecture.

## Privacy & Security

Privacy is one of the main principles behind NativeConvert.

- Files are intended to be processed locally.
- No mandatory file uploads.
- No mandatory cloud services.
- Original files are preserved by default.

The goal is to give users full control over their files and conversions.

## Getting Started

### Build requirements

- Rust toolchain
- Cargo
- Git (to clone the repository)

### Build and register the context menu (Windows)

```bash
git clone https://github.com/YOUR_USERNAME/NativeConvert.git
cd NativeConvert
cargo build --release
```

The context-menu registration requires `logo.ico` in the same folder as
`NativeConvert.exe`. Copy the supplied icon next to the release executable:

```powershell
Copy-Item assets\logo.ico target\release\logo.ico
```

Register the context-menu entries for the current Windows user:

```powershell
.\target\release\NativeConvert.exe --install
```

Remove them with:

```powershell
.\target\release\NativeConvert.exe --uninstall
```

`--install` checks that `logo.ico` exists before changing the registry and
reports an error if the file is missing. This command only registers the app;
it does not copy files or install a separate setup package. Keep the executable
and icon together after registration. No external conversion engine is needed
yet because file conversion has not been implemented.

The `package` directory contains an Appx manifest, but this repository does not
include a certificate for signing packages. With the Windows 10 SDK installed,
build an MSIX package using:

```powershell
.\package\build-msix.ps1
```

Without a signing certificate, this creates an **unsigned** package that
Windows cannot install. To create a local development certificate and sign the
package, run:

```powershell
$cert = New-SelfSignedCertificate `
    -Type Custom `
    -Subject "CN=NativeConvertDev" `
    -KeyUsage DigitalSignature `
    -CertStoreLocation "Cert:\CurrentUser\My" `
    -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3")

.\package\build-msix.ps1 -CertificateThumbprint $cert.Thumbprint
```

The script builds both Rust components and packages the executable, shell DLL,
icon, manifest, and package assets. When signing, it also exports the public
certificate as `package\dist\NativeConvertDev.cer` (the private key is not
exported). To install a locally signed package for testing, import that
certificate into the current user's Trusted People store and install the
package:

```powershell
Import-Certificate `
    -FilePath .\package\dist\NativeConvertDev.cer `
    -CertStoreLocation Cert:\CurrentUser\TrustedPeople
Add-AppxPackage .\package\dist\NativeConvert.Dev_1.0.0.0_x64.msix
```

Only trust certificates you created or otherwise verified. A self-signed
development certificate is only suitable for testing; distribute packages
signed with a certificate trusted by your users, or publish through the
Microsoft Store.

## Contributing

NativeConvert is an open-source project, and contributions are welcome.

Ideas, bug reports, feature requests and code contributions will help improve the project.

Contribution guidelines will be published as development progresses.

## License

NativeConvert is released under the MIT License.

See the [LICENSE](LICENSE) file for more information.

---

<p align="center">
  **NativeConvert**
  
  *Simple. Native. Private.*
</p>
