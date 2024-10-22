pub mod datasheet;
pub mod databook;
pub mod traits;
pub mod otp;

use crate::otp::OneTimePad;

use pyo3::prelude::*;

#[pymodule]
fn otp_exchange(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<OneTimePad>()?;
    Ok(())
}
