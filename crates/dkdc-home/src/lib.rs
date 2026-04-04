use std::path::PathBuf;

/// Returns the dkdc home directory.
///
/// Respects the `DKDC_HOME` environment variable. Falls back to `$HOME/.dkdc`.
pub fn home() -> std::io::Result<PathBuf> {
    if let Ok(custom) = std::env::var("DKDC_HOME") {
        if !custom.is_empty() {
            return Ok(PathBuf::from(custom));
        }
    }
    dirs::home_dir().map(|h| h.join(".dkdc")).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "could not determine home directory",
        )
    })
}

/// Returns `~/.dkdc/{subdir}`, creating the directory if it doesn't exist.
pub fn ensure(subdir: impl AsRef<std::path::Path>) -> std::io::Result<PathBuf> {
    let path = home()?.join(subdir);
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn home_returns_dkdc_dir() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { std::env::remove_var("DKDC_HOME") };
        let h = home().unwrap();
        assert!(h.ends_with(".dkdc"));
        assert!(h.is_absolute());
    }

    #[test]
    fn home_respects_env_override() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { std::env::set_var("DKDC_HOME", "/tmp/test-dkdc-home") };
        let h = home().unwrap();
        assert_eq!(h, PathBuf::from("/tmp/test-dkdc-home"));
        unsafe { std::env::remove_var("DKDC_HOME") };
    }

    #[test]
    fn home_ignores_empty_env() {
        let _lock = ENV_LOCK.lock().unwrap();
        unsafe { std::env::set_var("DKDC_HOME", "") };
        let h = home().unwrap();
        assert!(h.ends_with(".dkdc"));
        unsafe { std::env::remove_var("DKDC_HOME") };
    }

    #[test]
    fn ensure_creates_subdir() {
        let _lock = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("DKDC_HOME", tmp.path().to_str().unwrap()) };
        let db_dir = ensure("db").unwrap();
        assert!(db_dir.exists());
        assert!(db_dir.ends_with("db"));
        assert!(db_dir.starts_with(home().unwrap()));
        unsafe { std::env::remove_var("DKDC_HOME") };
    }

    #[test]
    fn home_path_can_be_created() {
        let _lock = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let custom = tmp.path().join("new-dkdc");
        unsafe { std::env::set_var("DKDC_HOME", custom.to_str().unwrap()) };
        let h = home().unwrap();
        std::fs::create_dir_all(&h).unwrap();
        assert!(h.exists());
        unsafe { std::env::remove_var("DKDC_HOME") };
    }

    #[test]
    fn ensure_creates_nested_subdirs() {
        let _lock = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("DKDC_HOME", tmp.path().to_str().unwrap()) };
        let nested = ensure("a/b/c").unwrap();
        assert!(nested.exists());
        assert!(nested.ends_with("a/b/c"));
        unsafe { std::env::remove_var("DKDC_HOME") };
    }

    #[test]
    fn ensure_idempotent() {
        let _lock = ENV_LOCK.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("DKDC_HOME", tmp.path().to_str().unwrap()) };
        let first = ensure("db").unwrap();
        let second = ensure("db").unwrap();
        assert_eq!(first, second);
        assert!(second.exists());
        unsafe { std::env::remove_var("DKDC_HOME") };
    }
}
