# dkdc-home

Tiny Rust library with Python bindings for resolving and creating the `~/.dkdc` home directory. Used by all dkdc projects that need local storage.

## architecture

```
crates/dkdc-home/       # pure Rust lib (no async, minimal deps)
  src/lib.rs             # home() -> PathBuf, ensure(subdir) -> Result<PathBuf>
crates/dkdc-home-py/    # PyO3 bindings (cdylib, excluded from workspace)
  src/lib.rs             # Python-facing wrappers
py/dkdc_home/           # Python wrapper + type stubs (core.pyi, py.typed)
```

Crates.io: `dkdc-home`. PyPI: `dkdc-home`. Installed binary: none (library only).

## development

```bash
bin/setup       # install rustup + uv if needed
bin/build       # build all (Rust + Python)
bin/build-rs    # build Rust crate
bin/build-py    # build Python bindings (maturin develop)
bin/check       # run all checks (format, lint, test)
bin/check-rs    # Rust checks (fmt, clippy, test)
bin/check-py    # Python checks (ruff, ty)
bin/format      # format all code
bin/format-rs   # format Rust
bin/format-py   # format Python (ruff)
bin/test        # run all tests
bin/test-rs     # Rust tests
bin/test-py     # Python tests (pytest)
bin/install-py  # install via uv tool
bin/build-wheels # build release wheels (maturin)
bin/build-sdist  # build source distribution
bin/bump-version # bump version (--patch, --minor (default), --major)
```

Rust checks: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
Python checks: `ruff check`, `ruff format --check`, `ty check`

## API

- `home() -> PathBuf` / `home() -> str` — returns `~/.dkdc`, respects `DKDC_HOME` env var
- `ensure(subdir) -> io::Result<PathBuf>` / `ensure(subdir) -> str` — returns `~/.dkdc/{subdir}`, creates it if needed

## conventions

- Rust stable toolchain (edition 2024)
- No async, no heavy dependencies
- `dirs` crate for cross-platform home directory resolution
- Python bindings via PyO3 + maturin
- uv for Python tooling (ruff, ty, pytest)
- py crate excluded from Cargo workspace (built by maturin)
