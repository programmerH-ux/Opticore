use pyo3::prelude::*;

mod license;
mod core;
#[cfg(feature = "pro")]
mod pro;

#[pyfunction]
fn activate(key: &str) -> PyResult<()> {
    if key == "OPTIPRO-2026" {
        license::activate_license();
        Ok(())
    } else {
        Err(pyo3::exceptions::PyValueError::new_err("Invalid license key"))
    }
}

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::gradient::GradientDescent>()?;

    #[cfg(feature = "pro")]
    {
        m.add_class::<pro::adam::Adam>()?;
        m.add_class::<pro::rmsprop::RMSProp>()?;
    }

    m.add_function(wrap_pyfunction!(activate, m)?)?;

    Ok(())
}
