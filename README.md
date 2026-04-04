# dkdc-home

[![GitHub Release](https://img.shields.io/github/v/release/dkdc-io/home?color=blue)](https://github.com/dkdc-io/home/releases)
[![crates.io](https://img.shields.io/crates/v/dkdc-home?color=blue)](https://crates.io/crates/dkdc-home)
[![PyPI](https://img.shields.io/pypi/v/dkdc-home?color=blue)](https://pypi.org/project/dkdc-home/)
[![CI](https://img.shields.io/github/actions/workflow/status/dkdc-io/home/ci.yml?branch=main&label=CI)](https://github.com/dkdc-io/home/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-8A2BE2.svg)](https://github.com/dkdc-io/home/blob/main/LICENSE)

Home library.

## Install

```bash
cargo add dkdc-home
```

```bash
uv add dkdc-home
```

## Usage

### Rust

```rust
let home = dkdc_home::home();
println!("{}", home.display());
```

Override with the `DKDC_HOME` environment variable:

```bash
DKDC_HOME=/custom/path cargo run
```

### Python

```python
import dkdc_home

home = dkdc_home.home()
print(home)
```
