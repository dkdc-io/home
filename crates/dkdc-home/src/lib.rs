use std::path::PathBuf;

/// Returns the dkdc home directory.
///
/// Respects the `DKDC_HOME` environment variable. Falls back to `$HOME/.dkdc`.
pub fn home() -> PathBuf {
    if let Ok(custom) = std::env::var("DKDC_HOME") {
        if !custom.is_empty() {
            return PathBuf::from(custom);
        }
    }
    dirs::home_dir()
        .expect("could not determine home directory")
        .join(".dkdc")
}

/// Returns `~/.dkdc/{subdir}`, creating the directory if it doesn't exist.
pub fn ensure(subdir: impl AsRef<std::path::Path>) -> std::io::Result<PathBuf> {
    let path = home().join(subdir);
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_returns_dkdc_dir() {
        unsafe { std::env::remove_var("DKDC_HOME") };
        let h = home();
        assert!(h.ends_with(".dkdc"));
    }

    #[test]
    fn home_respects_env_override() {
        unsafe { std::env::set_var("DKDC_HOME", "/tmp/test-dkdc-home") };
        let h = home();
        assert_eq!(h, PathBuf::from("/tmp/test-dkdc-home"));
        unsafe { std::env::remove_var("DKDC_HOME") };
    }

    #[test]
    fn ensure_creates_subdir() {
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("DKDC_HOME", tmp.path().to_str().unwrap()) };
        let db_dir = ensure("db").unwrap();
        assert!(db_dir.exists());
        assert!(db_dir.ends_with("db"));
        unsafe { std::env::remove_var("DKDC_HOME") };
    }
}
