# License: MIT
# Copyright © 2024 Frequenz Energy-as-a-Service GmbH

__all__ = "Resampler", "ResamplingFunction", "resample"

from datetime import datetime, timedelta
from enum import Enum, unique
from typing import Literal, Optional, Sequence

@unique
class ResamplingFunction(Enum):
    """
    The ResamplingFunction enum represents the different resampling functions
    that can be used to resample a time series.
    """

    Average = 0
    """Calculates the average of all samples in the time step (ignoring None values)"""
    Sum = 1
    """Calculates the sum of all samples in the time step (ignoring None values)"""
    Max = 2
    """Calculates the maximum of all samples in the time step"""
    Min = 3
    """Calculates the minimum of all samples in the time step"""
    Last = 4
    """Returns the last sample in the time step"""
    Count = 5
    """Counts the number of samples in the time step"""
    First = 6
    """Returns the first sample in the time step"""
    Coalesce = 7
    """Returns the first non-None sample in the time step"""

    @staticmethod
    def values() -> list[int]:
        """
        Returns a list of all values of the enum.

        Returns:
            A list of all values of the enum.
        """

    @staticmethod
    def members() -> list[tuple[str, int]]:
        """
        Returns a list of all members of the enum.

        Returns:
            A list of all members of the enum.
        """

class Resampler:
    """
    The Resampler class is used to resample a time series of samples.

    It stores the samples in a buffer and resamples the samples in the buffer when the
    resample method is called.
    A resampler can be configured with a resampling function and a resampling interval.
    """

    def __init__(
        self,
        interval: timedelta,
        resampling_function: ResamplingFunction,
        *,
        max_age_in_intervals: int,
        start: datetime,
        closed: Literal["left", "right"],
        label: Literal["left", "right"],
    ):
        """
        Initializes a new Resampler object.

        Args:
            interval: The resampling interval.
            resampling_function: The resampling function.
            max_age_in_intervals: The maximum age of a sample in intervals.
            start: The start time of the resampling.
            closed: Controls which interval edge is closed for sample
                membership. Use `"left"` for `[start, end)` intervals or
                `"right"` for `(start, end]` intervals.
            label: Controls the output timestamp labeling. Use `"left"` to
                label each interval with its start or `"right"` to label it
                with its end.
        """

    def push_sample(self, *, timestamp: datetime, value: Optional[float]) -> None:
        """
        Pushes a new sample into the resampler buffer.

        Args:
            timestamp: The timestamp of the sample.
            value: The value of the sample.
        """

    def resample(
        self, end: datetime | None = None
    ) -> list[tuple[datetime, Optional[float]]]:
        """
        Resamples the samples in the buffer until the given end time.

        Args:
            end: The end time of the resampling. If `None` the samples in the buffer will be
                resampled until the current date/time.

        Returns:
            A list of tuples with the resampled samples.
        """


def resample(
    data: Sequence[tuple[datetime, Optional[float]]],
    interval: timedelta,
    method: ResamplingFunction,
    *,
    closed: Literal["left", "right"],
    label: Literal["left", "right"],
) -> list[tuple[datetime, Optional[float]]]:
    """
    Resamples a list of timestamp/value pairs in a single call.

    This is a convenience function for one-shot resampling without needing to
    manage a `Resampler` instance.

    Args:
        data: A list of (timestamp, value) tuples to resample. Must be sorted by timestamp.
        interval: The resampling interval.
        method: The resampling function to use for aggregating values within each interval.
        closed: Which interval edge is closed for sample membership. Use
            `"left"` for `[start, end)` intervals or `"right"` for
            `(start, end]` intervals.
        label: Which interval edge to use for output timestamps. Use `"left"`
            for the interval start or `"right"` for the interval end.

    Returns:
        A list of (timestamp, value) tuples representing the resampled data.
    """
