# License: MIT
# Copyright © 2024 Frequenz Energy-as-a-Service GmbH

"""Tests to verify that the resampler can be used successfully from Python."""

import datetime as dt

from frequenz.resampling import Resampler, ResamplingFunction


def test_resampler_resampling_function_average() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 5 * step, 3.0),
        (start + 10 * step, 8.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_sum() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Sum,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 5 * step, 15.0),
        (start + 10 * step, 40.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_max() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Max,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 5 * step, 5.0),
        (start + 10 * step, 10.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_min() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Min,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 5 * step, 1.0),
        (start + 10 * step, 6.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_last() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Last,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 5 * step, 5.0),
        (start + 10 * step, 10.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_count() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Count,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 5 * step, 5.0),
        (start + 10 * step, 5.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampling_none() -> None:
    """Test resampling with None values."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 11):
        resampler.push_sample(timestamp=start + i * step, value=None)

    expected = [
        (start + 5 * step, None),
        (start + 10 * step, None),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_enum_values() -> None:
    """Test the ResamplingFunction enum."""
    assert ResamplingFunction.values() == [0, 1, 2, 3, 4, 5]


def test_enum_members() -> None:
    """Test the ResamplingFunction enum."""
    assert ResamplingFunction.members() == [
        ("Average", 0),
        ("Sum", 1),
        ("Max", 2),
        ("Min", 3),
        ("Last", 4),
        ("Count", 5),
    ]


def test_enum_str_repr() -> None:
    """Test the ResamplingFunction enum."""
    assert str(ResamplingFunction.Average) == "ResamplingFunction.Average"
    assert repr(ResamplingFunction.Average) == "<ResamplingFunction.Average: 0>"
    assert str(ResamplingFunction.Sum) == "ResamplingFunction.Sum"
    assert repr(ResamplingFunction.Sum) == "<ResamplingFunction.Sum: 1>"
    assert str(ResamplingFunction.Max) == "ResamplingFunction.Max"
    assert repr(ResamplingFunction.Max) == "<ResamplingFunction.Max: 2>"
    assert str(ResamplingFunction.Min) == "ResamplingFunction.Min"
    assert repr(ResamplingFunction.Min) == "<ResamplingFunction.Min: 3>"
    assert str(ResamplingFunction.Last) == "ResamplingFunction.Last"
    assert repr(ResamplingFunction.Last) == "<ResamplingFunction.Last: 4>"
    assert str(ResamplingFunction.Count) == "ResamplingFunction.Count"
    assert repr(ResamplingFunction.Count) == "<ResamplingFunction.Count: 5>"


def test_resampling_function_name_value() -> None:
    """Test the ResamplingFunction name and value interface."""
    assert ResamplingFunction.Average.name == "Average"
    assert ResamplingFunction.Average.value == 0
    assert ResamplingFunction.Sum.name == "Sum"
    assert ResamplingFunction.Sum.value == 1
    assert ResamplingFunction.Max.name == "Max"
    assert ResamplingFunction.Max.value == 2
    assert ResamplingFunction.Min.name == "Min"
    assert ResamplingFunction.Min.value == 3
    assert ResamplingFunction.Last.name == "Last"
    assert ResamplingFunction.Last.value == 4
    assert ResamplingFunction.Count.name == "Count"
    assert ResamplingFunction.Count.value == 5


def test_resampling_function_init() -> None:
    """Test the ResamplingFunction init."""
    assert ResamplingFunction(0) == ResamplingFunction.Average
    assert ResamplingFunction(1) == ResamplingFunction.Sum
    assert ResamplingFunction(2) == ResamplingFunction.Max
    assert ResamplingFunction(3) == ResamplingFunction.Min
    assert ResamplingFunction(4) == ResamplingFunction.Last
    assert ResamplingFunction(5) == ResamplingFunction.Count


def test_resampler_first_timestamp() -> None:
    """Test the resampler with the first timestamp."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=0.5)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=True,
    )

    for i in range(0, 20):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

    expected = [
        (start + 0 * step, 5.5),
        (start + 10 * step, 15.5),
    ]

    resampled = resampler.resample(start + 20 * step)

    assert resampled == expected


def test_resampler_last_timestamp() -> None:
    """Test the resampler with the last timestamp."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=0.5)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    for i in range(1, 21):
        resampler.push_sample(timestamp=start + i * step, value=i)

    expected = [
        (start + 10 * step, 5.5),
        (start + 20 * step, 15.5),
    ]

    resampled = resampler.resample(start + 20 * step)

    assert resampled == expected
