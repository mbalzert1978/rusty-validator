use pyo3::prelude::*;
use validator::ValidateEmail;
use validator::ValidateIp;
use validator::ValidateUrl;

#[pyfunction]
fn validate_email(email: String) -> PyResult<bool> {
    Ok(email.validate_email())
}

#[pyfunction]
fn validate_url(url: String) -> PyResult<bool> {
    Ok(url.validate_url())
}

#[pyfunction]
fn validate_ip(ip: String) -> PyResult<bool> {
    Ok(ip.validate_ip())
}

#[pymodule]
fn _validator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(validate_email, m)?)?;
    m.add_function(wrap_pyfunction!(validate_url, m)?)?;
    m.add_function(wrap_pyfunction!(validate_ip, m)?)?;
    Ok(())
}
