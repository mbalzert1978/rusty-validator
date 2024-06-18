use pyo3::prelude::*;
use validator::ValidateEmail;

#[pyfunction]
fn validate(email: String) -> PyResult<bool> {
    Ok(email.validate_email())
}

#[pymodule]
fn _email_validator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    Ok(())
}
