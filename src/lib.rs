use pyo3::prelude::*;

mod license;
mod core;
#[cfg(feature = "pro")]
mod pro;

#[pymodule]
fn optiengine(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<core::gradient::GradientDescent>()?;

    #[cfg(feature = "pro")]
    {
        m.add_class::<pro::adam::Adam>()?;
        m.add_class::<pro::rmsprop::RMSProp>()?;
    }

    Ok(())
}
