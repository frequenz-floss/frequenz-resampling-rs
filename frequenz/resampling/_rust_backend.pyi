# License: MIT
# Copyright © 2024 Frequenz Energy-as-a-Service GmbH

__all__ = "Resampler", "ResamplingFunction"

from datetime import datetime, timedelta
from enum import Enum, unique
from typing import Optional

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
        first_timestamp: bool = True,
    ):
        """
        Initializes a new Resampler object.

        Args:
            interval: The resampling interval.
            resampling_function: The resampling function.
            max_age_in_intervals: The maximum age of a sample in intervals.
            start: The start time of the resampling.
            first_timestamp: Whether the resampled timestamp should be the first
                timestamp in the buffer or the last timestamp in the buffer.
                Defaults to `True`.
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
