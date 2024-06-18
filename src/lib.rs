use pyo3::prelude::*;
use validator::ValidateEmail;

#[pyfunction]
fn email_validator(email: String) -> PyResult<bool> {
    Ok(email.validate_email())
}

#[pymodule]
fn validate_email(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(email_validator, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::IntoPyDict;
    use pyo3::Python;

    #[test]
    fn test_valid_email() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let validate_email = PyModule::new(py, "validate_email").unwrap();
            validate_email.add_function(wrap_pyfunction!(email_validator, validate_email).unwrap()).unwrap();

            let locals = [("validate_email", validate_email)].into_py_dict(py);
            let result: bool = py
                .eval("validate_email.email_validator('test@example.com')", None, Some(&locals))
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
            let validate_email = PyModule::new(py, "validate_email").unwrap();
            validate_email.add_function(wrap_pyfunction!(email_validator, validate_email).unwrap()).unwrap();

            let locals = [("validate_email", validate_email)].into_py_dict(py);
            let result: bool = py
                .eval("validate_email.email_validator('invalid-email')", None, Some(&locals))
                .unwrap()
                .extract()
                .unwrap();
            assert!(!result);
        });
    }
}