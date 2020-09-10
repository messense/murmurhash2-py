# -*- coding: utf-8 -*-

import pytest

from murmurhash2 import murmurhash2, murmurhash3


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


@pytest.mark.parametrize(
    "key, expected",
    [
        ("", 36859204),
        ("a", 3144985375),
        ("ab", 3262304301),
        ("abc", 476091040),
        ("abcd", 412992581),
        ("abcde", 2747833956),
        ("abcdefghijklmnop", 2078305053),
    ],
)
def test_murmurhash3(key, expected):
    assert murmurhash3(key.encode("utf-8"), SEED) == expected
