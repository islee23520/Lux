# linalab-lux

Cargo installer launcher for the LUX CLI.

This crate intentionally does not contain the LUX gateway source code. It installs a small `lux` launcher that resolves the matching prebuilt LUX binary from GitHub Releases and executes it.

```bash
cargo install linalab-lux
lux --help
```

Release assets are expected at:

```text
https://github.com/islee23520/lux/releases/latest/download/lux-<target-triple>
```

For local verification, set `LUX_INSTALLER_BINARY` to an existing `lux` binary path.
