# -*- coding: utf-8 -*-

import pytest

from murmurhash2 import murmurhash2


SEED = 3242157231


@pytest.mark.parametrize(
    "key, expected",
    [
        ("", 3632506080),
        ("a", 455683869),
        ("ab", 2448092234),
        ("abc", 2066295634),
        ("abcd", 2588571162),
        ("abcde", 2988696942),
        ("abcdefghijklmnop", 2350868870),
    ],
)
def test_murmurhash2(key, expected):
    assert murmurhash2(key.encode("utf-8"), SEED) == expected
