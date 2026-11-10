<div align="center">
<img src="assets/branding/moon-viewer-lockup.png" alt="Moon Viewer" width="320">
</div>

# Moon Viewer

Moon Viewer is an open-source Windows viewer for Second Life derived from
[Prototype](https://github.com/deadlyxreturn/Prototype) by Ryan Johnson. The
Moon Palace login scene and the lavender, pink and gold interface extend
from the login screen to Preferences, Conversations and other windows.

This software is not provided or supported by Linden Lab, the makers of Second Life.

## Downloads and support

- Verified packages, when available, appear on the [releases page](https://github.com/deadlyxreturn/MoonViewer/releases).
- Report bugs and ask for help through [GitHub Issues](https://github.com/deadlyxreturn/MoonViewer/issues). There is no dedicated customer support service.
- Review the [privacy policy](PRIVACY.md) and [viewer policy notes](docs/MOON-VIEWER-POLICY.md) before installing. The viewer retains Prototype's optional RLVa controls, third-party grid support and link previews. Imported media can depend on external plugins, screen recording needs FFmpeg, and OpenXR needs a compatible runtime and headset.
- The Windows installer is per user. It uses `%LOCALAPPDATA%\Programs\MoonViewer` by default and adds a Start menu entry and optional desktop shortcut. Windows Settings can uninstall it. The installer is unsigned.

## Build on Windows x64

Use a current Vulkan graphics driver and Visual Studio 2022 Build Tools with
the C++ workload and Windows SDK. Install Rustup with the
`x86_64-pc-windows-msvc` host, plus CMake and NSIS. The repository pins
Rust 1.99.1. Run the build script from a Developer PowerShell with Cargo,
CMake and MakeNSIS on the path.

```powershell
./scripts/build-release.ps1 -Tag v0.1.0
```

Cargo uses the checked-in lockfile. The build script places the installer,
portable ZIP, update policy and SHA-256 files under
`target/moon-viewer-dist/v0.1.0`. The same script runs in the Windows GitHub
Actions workflow. See [asset provenance](assets/branding/MOON-VIEWER.md)
for the input images and icon regeneration recipe.

The local viewer build and test commands use the CI profile.

```powershell
cargo build --locked --profile ci -p prototype-viewer --bin moon-viewer
cargo test --locked --profile ci --workspace
```

The package keeps the inherited Cargo crate names. The viewer executable
is `moon-viewer.exe`. The local build above uses the system OpenXR loader
when available. The Windows release script builds with the static loader.

## Licence and source

Moon Viewer is distributed under the GNU General Public License version 3
or later. The fork started at Prototype revision
[`eede23767d`](https://github.com/deadlyxreturn/Prototype/commit/eede23767d4421f182ee4f0ab60f7eccaf417cbd).
The [GPL text](LICENSE) and [original credits and notices](NOTICE.md)
document its lineage and third-party components.

Second Life® is a trademark of Linden Research, Inc.
