use pyo3::prelude::*;

mod license;

#[pyfunction]
fn activate_license(key: &str) -> PyResult<()> {
    license::activate_license_py(key)
}

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(activate_license, m)?)?;
    Ok(())
}
