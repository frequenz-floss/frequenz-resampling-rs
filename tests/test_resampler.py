# License: MIT
# Copyright © 2024 Frequenz Energy-as-a-Service GmbH

"""Tests to verify that the resampler can be used successfully from Python."""

import datetime as dt
from typing import Literal

import pandas as pd
import pytest
from frequenz.resampling import Closed, Label, Resampler, ResamplingFunction, resample


def test_resampler_resampling_function_average() -> None:
    """Test the resampler."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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
        closed=Closed.Left,
        label=Label.Right,
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


def test_resampler_label_left() -> None:
    """Test the resampler with the left interval label."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=0.5)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        closed=Closed.Left,
        label=Label.Left,
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
    """Test the resampler with the right interval label."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=0.5)
    resampler = Resampler(
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        max_age_in_intervals=1,
        start=start,
        closed=Closed.Left,
        label=Label.Right,
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

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=Closed.Left,
        label=Label.Left,
    )

    assert len(result) == 2
    assert result[0] == (start, 3.0)
    assert result[1] == (start + 5 * step, 8.0)


def test_resample_function_label_right() -> None:
    """Test the resample function with label='right'."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data = [(start + i * step, float(i + 1)) for i in range(10)]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=Closed.Left,
        label=Label.Right,
    )

    assert len(result) == 2
    # With label='right', timestamps are at end of interval
    assert result[0] == (start + 5 * step, 3.0)
    assert result[1] == (start + 10 * step, 8.0)


def test_resample_function_closed_right() -> None:
    """Test the resample function with right-closed intervals."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    data = [
        (start, 10.0),
        (start + 5 * step, 20.0),
    ]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Sum,
        closed=Closed.Right,
        label=Label.Right,
    )

    assert result == [
        (start, 10.0),
        (start + 5 * step, 20.0),
    ]


@pytest.mark.parametrize(
    ("closed", "label", "pandas_closed", "pandas_label"),
    [
        (Closed.Left, Label.Left, "left", "left"),
        (Closed.Left, Label.Right, "left", "right"),
        (Closed.Right, Label.Left, "right", "left"),
        (Closed.Right, Label.Right, "right", "right"),
    ],
)
def test_resample_function_matches_pandas(
    closed: Closed,
    label: Label,
    pandas_closed: Literal["left", "right"],
    pandas_label: Literal["left", "right"],
) -> None:
    """Test the one-shot API against pandas with matching closed/label settings."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)
    data = [(start + i * step, float(i + 1)) for i in range(10)]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=closed,
        label=label,
    )

    pandas_series = pd.Series(
        [value for _, value in data],
        index=pd.DatetimeIndex([timestamp for timestamp, _ in data]),
        dtype="float64",
    )
    pandas_result = pandas_series.resample(
        "5s", closed=pandas_closed, label=pandas_label
    ).mean()
    expected = [
        (timestamp.to_pydatetime(), float(value))
        for timestamp, value in zip(
            pandas_result.index, pandas_result.to_list(), strict=True
        )
    ]

    assert result == expected


def test_label_init_invalid_value() -> None:
    """Test the Label enum rejects invalid values."""
    with pytest.raises(ValueError, match="Invalid label"):
        Label(99)


def test_closed_init_invalid_value() -> None:
    """Test the Closed enum rejects invalid values."""
    with pytest.raises(ValueError, match="Invalid closed"):
        Closed(99)


def test_label_values_members() -> None:
    """Test the Label enum helpers."""
    assert Label.values() == [0, 1]
    assert Label.members() == [("Left", 0), ("Right", 1)]


def test_closed_values_members() -> None:
    """Test the Closed enum helpers."""
    assert Closed.values() == [0, 1]
    assert Closed.members() == [("Left", 0), ("Right", 1)]


def test_resample_function_empty_data() -> None:
    """Test the resample function with empty data."""
    data: list[tuple[dt.datetime, float | None]] = []

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=Closed.Left,
        label=Label.Left,
    )

    assert result == []


def test_resample_function_with_none_values() -> None:
    """Test the resample function with None values."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    # First value in each interval is None
    data: list[tuple[dt.datetime, float | None]] = [
        (start + i * step, None if i in (0, 5) else float(i + 1)) for i in range(10)
    ]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=Closed.Left,
        label=Label.Left,
    )

    assert len(result) == 2
    # Interval [0, 5): values 2,3,4,5 → avg = 3.5
    # Interval [5, 10): values 7,8,9,10 → avg = 8.5
    assert result[0] == (start, 3.5)
    assert result[1] == (start + 5 * step, 8.5)


def test_resample_function_interval_with_only_none() -> None:
    """Test a bucket that contains only None values."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data: list[tuple[dt.datetime, float | None]] = [
        (start + i * step, None if i < 5 else float(i + 1)) for i in range(10)
    ]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=Closed.Left,
        label=Label.Left,
    )

    assert len(result) == 2
    assert result[0] == (start, None)
    assert result[1] == (start + 5 * step, 8.0)


def test_resample_function_sum() -> None:
    """Test the resample function with Sum method."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
    step = dt.timedelta(seconds=1)

    data = [(start + i * step, float(i + 1)) for i in range(10)]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Sum,
        closed=Closed.Left,
        label=Label.Left,
    )

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

    min_result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Min,
        closed=Closed.Left,
        label=Label.Left,
    )
    max_result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Max,
        closed=Closed.Left,
        label=Label.Left,
    )

    assert min_result[0] == (start, 1.0)
    assert min_result[1] == (start + 5 * step, 6.0)
    assert max_result[0] == (start, 5.0)
    assert max_result[1] == (start + 5 * step, 10.0)


def test_resample_function_single_sample() -> None:
    """Test the resample function with a single sample."""
    start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)

    data = [(start, 42.0)]

    result = resample(
        data,
        dt.timedelta(seconds=5),
        ResamplingFunction.Average,
        closed=Closed.Left,
        label=Label.Left,
    )

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
        result = resample(
            data,
            dt.timedelta(seconds=5),
            method,
            closed=Closed.Left,
            label=Label.Left,
        )
        assert len(result) == 1
        assert result[0][0] == start
