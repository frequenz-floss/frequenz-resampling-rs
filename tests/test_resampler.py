# License: MIT
# Copyright © 2024 Frequenz Energy-as-a-Service GmbH

"""Tests to verify that the resampler can be used successfully from Python."""

import datetime as dt

from frequenz.resampling import Resampler, ResamplingFunction, resample


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

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → avg = 3.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → avg = 8.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

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

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → sum = 15.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → sum = 40.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

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

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → max = 5.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → max = 10.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

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

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → min = 1.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → min = 6.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

    expected = [
        (start + 5 * step, 1.0),
        (start + 10 * step, 6.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_first() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.First,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → first = 1.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → first = 6.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

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

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → last = 5.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → last = 10.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

    expected = [
        (start + 5 * step, 5.0),
        (start + 10 * step, 10.0),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_resampler_resampling_function_coalesce() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Coalesce,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    # Data starts at t=0 with values 1-10, but t=5 is None
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → coalesce = 1.0
    # Interval [5, 10): t=5,6,7,8,9 with values None,7,8,9,10 → coalesce = 7.0
    for i in range(10):
        if i == 5:
            resampler.push_sample(timestamp=start + i * step, value=None)
        else:
            resampler.push_sample(timestamp=start + i * step, value=i + 1)

    expected = [
        (start + 5 * step, 1.0),
        (start + 10 * step, 7.0),
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

    # Data starts at t=0 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 → count = 5.0
    # Interval [5, 10): t=5,6,7,8,9 → count = 5.0
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

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

    # All values are None
    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=None)

    expected = [
        (start + 5 * step, None),
        (start + 10 * step, None),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected


def test_enum_values() -> None:
    """Test the ResamplingFunction enum."""
    assert ResamplingFunction.values() == [0, 1, 2, 3, 4, 5, 6, 7]


def test_enum_members() -> None:
    """Test the ResamplingFunction enum."""
    assert ResamplingFunction.members() == [
        ("Average", 0),
        ("Sum", 1),
        ("Max", 2),
        ("Min", 3),
        ("Last", 4),
        ("Count", 5),
        ("First", 6),
        ("Coalesce", 7),
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
    assert str(ResamplingFunction.First) == "ResamplingFunction.First"
    assert repr(ResamplingFunction.First) == "<ResamplingFunction.First: 6>"
    assert str(ResamplingFunction.Coalesce) == "ResamplingFunction.Coalesce"
    assert repr(ResamplingFunction.Coalesce) == "<ResamplingFunction.Coalesce: 7>"


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
    assert ResamplingFunction.First.name == "First"
    assert ResamplingFunction.First.value == 6
    assert ResamplingFunction.Coalesce.name == "Coalesce"
    assert ResamplingFunction.Coalesce.value == 7


def test_resampling_function_init() -> None:
    """Test the ResamplingFunction init."""
    assert ResamplingFunction(0) == ResamplingFunction.Average
    assert ResamplingFunction(1) == ResamplingFunction.Sum
    assert ResamplingFunction(2) == ResamplingFunction.Max
    assert ResamplingFunction(3) == ResamplingFunction.Min
    assert ResamplingFunction(4) == ResamplingFunction.Last
    assert ResamplingFunction(5) == ResamplingFunction.Count
    assert ResamplingFunction(6) == ResamplingFunction.First
    assert ResamplingFunction(7) == ResamplingFunction.Coalesce


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
    """Test the resampler with the last timestamp (first_timestamp=False)."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=0.5)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        first_timestamp=False,
    )

    # Data starts at t=0, step=0.5s, 20 samples
    # Interval [0, 5): t=0,0.5,1,1.5,2,2.5,3,3.5,4,4.5 → values 1-10 → avg = 5.5
    # Interval [5, 10): t=5,5.5,6,6.5,7,7.5,8,8.5,9,9.5 → values 11-20 → avg = 15.5
    for i in range(20):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

    expected = [
        (start + 10 * step, 5.5),
        (start + 20 * step, 15.5),
    ]

    resampled = resampler.resample(start + 20 * step)

    assert resampled == expected


# Tests for the one-shot resample function


def test_resample_function_basic() -> None:
    """Test the resample function with basic usage."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    # Data: t=0,1,2,3,4,5,6,7,8,9 with values 1-10
    # Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → avg = 3.0
    # Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → avg = 8.0
    data = [(start + i * step, float(i + 1)) for i in range(10)]

    result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Average)

    assert len(result) == 2
    assert result[0] == (start, 3.0)
    assert result[1] == (start + 5 * step, 8.0)


def test_resample_function_first_timestamp_false() -> None:
    """Test the resample function with first_timestamp=False."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data = [(start + i * step, float(i + 1)) for i in range(10)]

    result = resample(
        data, dt.timedelta(seconds=5), ResamplingFunction.Average, first_timestamp=False
    )

    assert len(result) == 2
    # With first_timestamp=False, timestamps are at end of interval
    assert result[0] == (start + 5 * step, 3.0)
    assert result[1] == (start + 10 * step, 8.0)


def test_resample_function_empty_data() -> None:
    """Test the resample function with empty data."""
    data: list[tuple[dt.datetime, float | None]] = []

    result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Average)

    assert result == []


def test_resample_function_with_none_values() -> None:
    """Test the resample function with None values."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    # First value in each interval is None
    data: list[tuple[dt.datetime, float | None]] = [
        (start + i * step, None if i in (0, 5) else float(i + 1)) for i in range(10)
    ]

    result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Average)

    assert len(result) == 2
    # Interval [0, 5): values 2,3,4,5 → avg = 3.5
    # Interval [5, 10): values 7,8,9,10 → avg = 8.5
    assert result[0] == (start, 3.5)
    assert result[1] == (start + 5 * step, 8.5)


def test_resample_function_sum() -> None:
    """Test the resample function with Sum method."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data = [(start + i * step, float(i + 1)) for i in range(10)]

    result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Sum)

    assert len(result) == 2
    # Interval [0, 5): sum(1,2,3,4,5) = 15.0
    # Interval [5, 10): sum(6,7,8,9,10) = 40.0
    assert result[0] == (start, 15.0)
    assert result[1] == (start + 5 * step, 40.0)


def test_resample_function_min_max() -> None:
    """Test the resample function with Min and Max methods."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data = [(start + i * step, float(i + 1)) for i in range(10)]

    min_result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Min)
    max_result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Max)

    assert min_result[0] == (start, 1.0)
    assert min_result[1] == (start + 5 * step, 6.0)
    assert max_result[0] == (start, 5.0)
    assert max_result[1] == (start + 5 * step, 10.0)


def test_resample_function_single_sample() -> None:
    """Test the resample function with a single sample."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)

    data = [(start, 42.0)]

    result = resample(data, dt.timedelta(seconds=5), ResamplingFunction.Average)

    assert len(result) == 1
    assert result[0] == (start, 42.0)


def test_resample_function_all_methods() -> None:
    """Test the resample function with all resampling methods."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data = [(start + i * step, float(i + 1)) for i in range(5)]

    # Test all methods work without errors
    for method in [
        ResamplingFunction.Average,
        ResamplingFunction.Sum,
        ResamplingFunction.Min,
        ResamplingFunction.Max,
        ResamplingFunction.First,
        ResamplingFunction.Last,
        ResamplingFunction.Count,
        ResamplingFunction.Coalesce,
    ]:
        result = resample(data, dt.timedelta(seconds=5), method)
        assert len(result) == 1
        assert result[0][0] == start
