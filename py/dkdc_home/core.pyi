def home() -> str:
    """Return the dkdc home directory path (~/.dkdc). Respects DKDC_HOME env var. Raises RuntimeError on failure."""
    ...

def ensure(subdir: str) -> str:
    """Return ~/.dkdc/{subdir}, creating the directory if needed. Raises RuntimeError on failure."""
    ...
