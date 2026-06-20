# Rusty Validator

This project provides a simple validation library written in Rust, with Python bindings using PyO3. The library uses the [`validator`](https://github.com/Keats/validator) crate to validate email addresses, URLs, and IP addresses.

## Features

- Validate email addresses,
- Validate URLs,
- Validate IP addresses using Python.

## Requirements

- Python 3.13 or higher

## Installation

You can install the package directly from PyPI:

```sh
pip install rusty-validator
```

## Usage

Here's an example of how to use the validation functions in Python:

```python
from rusty_validator import validate_email, validate_url, validate_ip

email = "example@example.com"
is_valid_email = validate_email(email)
print(f"Is valid email: {is_valid_email}")

url = "https://example.com"
is_valid_url = validate_url(url)
print(f"Is valid URL: {is_valid_url}")

ip = "127.0.0.1"
is_valid_ip = validate_ip(ip)
print(f"Is valid IP: {is_valid_ip}")
```

## Development

This project uses [uv](https://docs.astral.sh/uv/) to manage the virtual environment,
dev dependencies, and the pinned Python version (`.python-version`).
[maturin](https://www.maturin.rs/) remains the build backend that compiles the Rust
extension — the two are complementary: uv layers on top, maturin stays in `[build-system]`.

### Prerequisites

- [uv](https://docs.astral.sh/uv/getting-started/installation/)
- A [Rust toolchain](https://rustup.rs/) (cargo/rustc) to compile the extension

### Setup

```sh
uv sync                          # create .venv + install dev tools (maturin, pytest, mypy) from uv.lock
uv run maturin develop           # compile the Rust extension into the venv
```

After every change to `src/lib.rs`, re-run `uv run maturin develop` — uv does **not**
recompile compiled projects automatically.

### Test & type-check

```sh
uv run pytest                    # Python tests
uv run mypy python               # type-check stubs/sources
```

### Build a wheel

```sh
uv run maturin build --release   # outputs to target/wheels/
```

## License

This project is licensed under the MIT License.

## Acknowledgements

- [PyO3](https://github.com/PyO3/pyo3) for providing the Rust bindings for Python.
- [Validator](https://github.com/Keats/validator) for the validation functionality.
