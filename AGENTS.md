# dkdc-home

Tiny Rust library for resolving and creating the `~/.dkdc` home directory. Used by all dkdc projects that need local storage.

## architecture

```
crates/dkdc-home/       # pure Rust lib (no async, minimal deps)
  src/lib.rs             # home() -> PathBuf, ensure(subdir) -> Result<PathBuf>
```

Crates.io: `dkdc-home`. Installed binary: none (library only).

## development

```bash
bin/setup       # install rustup if needed
bin/build       # build Rust (bin/build-rs)
bin/check       # lint + test (bin/check-rs)
bin/format      # auto-format (bin/format-rs)
bin/test        # run tests (bin/test-rs)
```

Rust checks: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`

## API

- `home() -> PathBuf` — returns `~/.dkdc`, respects `DKDC_HOME` env var
- `ensure(subdir) -> io::Result<PathBuf>` — returns `~/.dkdc/{subdir}`, creates it if needed

## conventions

- Rust stable toolchain (edition 2024)
- No async, no heavy dependencies
- `dirs` crate for cross-platform home directory resolution
