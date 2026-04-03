# dkdc-home

Find and create the `~/.dkdc` home directory.

## usage

```rust
use dkdc_home::{home, ensure};

// Get the dkdc home directory
let h = home(); // ~/.dkdc

// Get a subdirectory, creating it if needed
let db_dir = ensure("db").unwrap(); // ~/.dkdc/db/
```

Set `DKDC_HOME` to override the default location.
