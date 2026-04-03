use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyfunction]
fn home() -> String {
    dkdc_home::home().to_string_lossy().into_owned()
}

#[pyfunction]
fn ensure(subdir: &str) -> PyResult<String> {
    dkdc_home::ensure(subdir)
        .map(|p| p.to_string_lossy().into_owned())
        .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))
}

// -- Module -------------------------------------------------------------------

#[pymodule]
mod core {
    use super::*;

    #[pymodule_init]
    fn module_init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(home, m)?)?;
        m.add_function(wrap_pyfunction!(ensure, m)?)?;
        Ok(())
    }
}
