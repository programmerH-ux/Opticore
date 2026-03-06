use pyo3::prelude::*;

mod license;
mod pro;

use pro::adam::Adam;
use pro::rmsprop::RMSProp;

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {

    // License functions
    m.add_function(wrap_pyfunction!(license::activate_license_py, m)?)?;
    m.add_function(wrap_pyfunction!(license::machine_id_py, m)?)?;

    // Optimizers
    m.add_class::<Adam>()?;
    m.add_class::<RMSProp>()?;

    Ok(())
}