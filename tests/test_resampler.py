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
    )

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
    )

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
    )

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
    )

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
    )

    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=i + 1)

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
    )

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
    )

    for i in range(10):
        resampler.push_sample(timestamp=start + i * step, value=None)

    expected = [
        (start + 5 * step, None),
        (start + 10 * step, None),
    ]

    resampled = resampler.resample(start + 10 * step)

    assert resampled == expected
