import pytest
from rusty_validator import validate_ip


@pytest.mark.parametrize(
    "input,expected",
    [
        ("1.1.1.1", True),
        ("255.0.0.0", True),
        ("0.0.0.0", True),
        ("256.1.1.1", False),
        ("25.1.1.", False),
        ("25,1,1,1", False),
        ("fe80::223:6cff:fe8a:2e8a", True),
        ("::ffff:254.42.16.14", True),
        ("2a02::223:6cff :fe8a:2e8a", False),
    ],
    ids=[
        "validate_ip_when_valid_ipv4_then_true",
        "validate_ip_when_valid_ipv4_then_true",
        "validate_ip_when_valid_ipv4_then_true",
        "validate_ip_when_invalid_ipv4_then_false",
        "validate_ip_when_invalid_ipv4_then_false",
        "validate_ip_when_invalid_ipv4_then_false",
        "validate_ip_when_valid_ipv6_then_true",
        "validate_ip_when_valid_ipv6_then_true",
        "validate_ip_when_invalid_ipv6_then_false",
    ],
)
def test_validate_ip(input: str, expected: bool) -> None:
    assert validate_ip(input) == expected


@pytest.mark.parametrize(
    "input,expected",
    [
        ("1.1.1.1", True),
        ("255.0.0.0", True),
        ("0.0.0.0", True),
        ("256.1.1.1", False),
        ("25.1.1.", False),
        ("25,1,1,1", False),
        ("25.1 .1.1", False),
        ("1.1.1.1\n", False),
        ("٧.2٥.3٣.243", False),
    ],
    ids=[
        "validate_ipv4_when_valid_ipv4_then_true",
        "validate_ipv4_when_valid_ipv4_then_true",
        "validate_ipv4_when_valid_ipv4_then_true",
        "validate_ipv4_when_invalid_ipv4_then_false",
        "validate_ipv4_when_invalid_ipv4_then_false",
        "validate_ipv4_when_invalid_ipv4_then_false",
        "validate_ipv4_when_invalid_ipv4_then_false",
        "validate_ipv4_when_invalid_ipv4_then_false",
        "validate_ipv4_when_invalid_ipv4_then_false",
    ],
)
def test_validate_ip_v4(input: str, expected: bool) -> None:
    assert validate_ip(input) == expected


@pytest.mark.parametrize(
    "input,expected",
    [
        ("fe80::223:6cff:fe8a:2e8a", True),
        ("2a02::223:6cff:fe8a:2e8a", True),
        ("1::2:3:4:5:6:7", True),
        ("::", True),
        ("::a", True),
        ("2::", True),
        ("::ffff:254.42.16.14", True),
        ("::ffff:0a0a:0a0a", True),
        ("::254.42.16.14", True),
        ("::0a0a:0a0a", True),
        ("foo", False),
        ("12345::", False),
        ("1::2::3::4", False),
        ("1::zzz", False),
        ("1:2", False),
        ("fe80::223: 6cff:fe8a:2e8a", False),
        ("2a02::223:6cff :fe8a:2e8a", False),
        ("::ffff:999.42.16.14", False),
        ("::ffff:zzzz:0a0a", False),
    ],
    ids=[
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_valid_ipv6_then_true",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
        "validate_ipv6_when_invalid_ipv6_then_false",
    ],
)
def test_validate_ip_v6(input: str, expected: bool) -> None:
    assert validate_ip(input) == expected
