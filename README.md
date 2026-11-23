
# NativeConvert

**Convert files. Not websites.**

NativeConvert is an open-source, native file conversion tool designed to make file conversion simple, fast and accessible directly from your operating system.

Instead of relying on online conversion websites, NativeConvert aims to provide a seamless experience through your system's native file manager.

Right-click a file, choose your desired output format, and let NativeConvert handle the rest.

---

## Why NativeConvert?

File conversion is often unnecessarily complicated.

Users frequently need to:
- Find an online conversion website.
- Upload their files.
- Wait for processing.
- Download the converted files.

NativeConvert aims to eliminate these unnecessary steps by providing a native, local-first alternative.

No browser required. No mandatory cloud services. Just file conversion directly on your machine.

## Core Features

The following features are planned for NativeConvert:

- **Native Integration** — Convert files directly from your operating system's context menu.
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
- [ ] Windows context menu integration.
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

NativeConvert is currently under development.

Build and usage instructions will be added as the first functional version becomes available.

### Requirements

- Rust toolchain
- Cargo
- Git

### Build

```bash
git clone https://github.com/YOUR_USERNAME/NativeConvert.git
cd NativeConvert
cargo build --release
```

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
