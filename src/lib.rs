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

#[cfg(test)]
mod tests {
    use pyo3::types::IntoPyDict;

    use super::*;

    #[test]
    fn test_valid_email() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let rusty_validator = PyModule::new_bound(py, "_validator").unwrap();
            _validator(&rusty_validator).unwrap();

            let locals = [("rusty_validator", rusty_validator)].into_py_dict_bound(py);
            let result: bool = py
                .eval_bound(
                    "rusty_validator.validate_email('example@example.com')",
                    None,
                    Some(&locals),
                )
                .unwrap()
                .extract()
                .unwrap();
            assert!(result);
        });
    }

    #[test]
    fn test_invalid_email() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let rusty_validator = PyModule::new_bound(py, "_validator").unwrap();
            _validator(&rusty_validator).unwrap();

            let locals = [("rusty_validator", rusty_validator)].into_py_dict_bound(py);
            let result: bool = py
                .eval_bound(
                    "rusty_validator.validate_email('invalid-email')",
                    None,
                    Some(&locals),
                )
                .unwrap()
                .extract()
                .unwrap();
            assert!(!result);
        });
    }
}
