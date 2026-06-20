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

// The #[pyfunction]s are plain Rust fns too, so we test the wrappers directly — no
// embedded interpreter needed. The Python-level integration (module registration, name
// binding) is covered by the pytest suite against the real built extension.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert!(validate_email("example@example.com".to_string()).unwrap());
    }

    #[test]
    fn test_invalid_email() {
        assert!(!validate_email("invalid-email".to_string()).unwrap());
    }

    #[test]
    fn test_valid_url() {
        assert!(validate_url("https://example.com".to_string()).unwrap());
    }

    #[test]
    fn test_invalid_url() {
        assert!(!validate_url("invalid-url".to_string()).unwrap());
    }

    #[test]
    fn test_valid_ip() {
        assert!(validate_ip("127.0.0.1".to_string()).unwrap());
    }

    #[test]
    fn test_invalid_ip() {
        assert!(!validate_ip("999.999.999.999".to_string()).unwrap());
    }
}
