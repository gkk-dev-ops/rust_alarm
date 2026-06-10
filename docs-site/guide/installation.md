# Installation

`clck` officially supports macOS and Linux. Windows support is best-effort.

## Cargo

Rust users can install the current published version from crates.io:

```bash
cargo install clck --locked
```

After installation, run `clck --help` to verify that the executable is on
`PATH`.

## GitHub Releases

Prebuilt binaries are published on the
[latest GitHub Release](https://github.com/gkk-dev-ops/clck/releases/latest).
Download the archive for your platform:

| Platform | Archive |
| --- | --- |
| Apple Silicon macOS | `clck-vX.Y.Z-macos-aarch64.tar.gz` |
| Intel macOS | `clck-vX.Y.Z-macos-x86_64.tar.gz` |
| ARM64 Linux | `clck-vX.Y.Z-linux-aarch64-musl.tar.gz` |
| x86-64 Linux | `clck-vX.Y.Z-linux-x86_64-musl.tar.gz` |
| x86-64 Windows | `clck-vX.Y.Z-windows-x86_64.zip` |

Each release includes `SHA256SUMS`. Verify the archive before extracting it:

```bash
# Linux
sha256sum -c SHA256SUMS

# macOS
shasum -a 256 -c SHA256SUMS
```

On Windows, compare `Get-FileHash -Algorithm SHA256` with the corresponding
entry in `SHA256SUMS`.

Extract the archive and place `clck` or `clck.exe` in a directory on `PATH`.

## Optional FFmpeg Support

MP3, WAV, FLAC, OGG, and AIFF play natively. Other custom sound formats,
including MP4, require `ffmpeg` to be installed and available on `PATH`.
