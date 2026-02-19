use pyo3::prelude::*;

mod license;
mod pro;

#[pyfunction]
fn activate_license(key: &str) -> PyResult<()> {
    license::activate_license_py(key)
}

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(activate_license, m)?)?;
    m.add_class::<pro::adam::Adam>()?;  // ADD THIS
    Ok(())
}
