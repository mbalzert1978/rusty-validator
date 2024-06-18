import pytest
from rusty_validator import validate_url


@pytest.mark.parametrize(
    "url,expected",
    [
        ("http", False),
        ("https://google.com", True),
        ("http://localhost:80", True),
        ("ftp://localhost:80", True),
    ],
    ids=[
        "validate_url_when_http_then_false",
        "validate_url_when_https_google_com_then_true",
        "validate_url_when_http_localhost_80_then_true",
        "validate_url_when_ftp_localhost_80_then_true",
    ],
)
def test_validate_url(url: str, expected: bool) -> None:
    assert validate_url(url) == expected
