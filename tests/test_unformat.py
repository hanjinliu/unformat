from unformat import compile
import pytest


@pytest.mark.parametrize(
    "pattern, string, expected",
    [
        ("{month}_{day}", "Jan_1", ["Jan", "1"]),
        ("{a}.{b}.{c}", "1.2.3", ["1", "2", "3"]),
        ("{a}.{b:int}.{c}", "1.2.3", ["1", 2, "3"]),
    ],
)
def test_unformat_single(pattern: str, string: str, expected: list):
    assert compile(pattern).unformat(string) == expected


def test_unformat_all():
    ptn = compile("{month}_{day}")
    assert ptn.unformat_all(["Jan_1", "Feb_2"]) == [
        ["Jan", "1"],
        ["Feb", "2"],
    ]


def test_unformat_to_dict():
    ptn = compile("{month}_{day}")
    assert ptn.unformat_to_dict(["Jan_1", "Feb_2"]) == {
        "month": ["Jan", "Feb"],
        "day": ["1", "2"],
    }
