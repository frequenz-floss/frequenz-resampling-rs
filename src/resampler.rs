// License: MIT
// Copyright © 2024 Frequenz Energy-as-a-Service GmbH

//! The resampler module provides the Resampler struct that is used to resample
//! a time series of samples.

use chrono::{DateTime, TimeDelta, Utc};
use log::warn;
use num_traits::FromPrimitive;
use std::fmt::Debug;
use std::ops::Div;

use itertools::Itertools;

pub type CustomResamplingFunction<S, T> = Box<dyn FnMut(&[&S]) -> Option<T> + Send + Sync>;

/// The Sample trait represents a single sample in a time series.
pub trait Sample: Clone + Debug + Default {
    type Value;
    fn new(timestamp: DateTime<Utc>, value: Option<Self::Value>) -> Self;
    fn timestamp(&self) -> DateTime<Utc>;
    fn value(&self) -> Option<Self::Value>;
}

/// The ResamplingFunction enum represents the different resampling functions
/// that can be used to resample a channel.
#[derive(Default)]
pub enum ResamplingFunction<
    T: Div<Output = T> + std::iter::Sum + Default + Debug,
    S: Sample<Value = T>,
> {
    /// Calculates the average of all samples in the time step (ignoring None
    /// values)
    #[default]
    Average,
    /// Calculates the sum of all samples in the time step (ignoring None
    /// values)
    Sum,
    /// Calculates the maximum value of all samples in the time step (ignoring
    /// None values)
    Max,
    /// Calculates the minimum value of all samples in the time step (ignoring
    /// None values)
    Min,
    /// Uses the first sample in the time step. If the first sample is None, the
    /// resampling function will return None.
    First,
    /// Uses the last sample in the time step. If the last sample is None, the
    /// resampling function will return None.
    Last,
    /// Returns the first non-None sample in the time step. If all samples are
    /// None, the resampling function will return None.
    Coalesce,
    /// Counts the number of samples in the time step (ignoring None values)
    Count,
    /// A custom resampling function that takes a closure that takes a slice of
    /// samples and returns an optional value.
    Custom(CustomResamplingFunction<S, T>),
}

impl<
        T: Div<Output = T> + std::iter::Sum + PartialOrd + FromPrimitive + Default + Debug,
        S: Sample<Value = T>,
    > ResamplingFunction<T, S>
{
    pub fn apply(&mut self, samples: &[&S]) -> Option<T> {
        match self {
            Self::Average => Self::Sum
                .apply(samples)
                .and_then(|sum| Self::Count.apply(samples).map(|count| sum.div(count))),
            Self::Sum => samples.iter().filter_map(|s| s.value()).sum1(),
            Self::Max => samples.iter().filter_map(|s| s.value()).max_by(|a, b| {
                a.partial_cmp(b).unwrap_or_else(|| {
                    if a.partial_cmp(&T::default()).is_some() {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                })
            }),
            Self::Min => samples.iter().filter_map(|s| s.value()).min_by(|a, b| {
                a.partial_cmp(b).unwrap_or_else(|| {
                    if a.partial_cmp(&T::default()).is_some() {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                })
            }),
            Self::First => samples.first().and_then(|s| s.value()),
            Self::Last => samples.last().and_then(|s| s.value()),
            Self::Coalesce => samples.iter().find_map(|s| s.value()),
            Self::Count => Some(
                T::from_usize(samples.iter().filter_map(|s| s.value()).count())
                    .unwrap_or_else(|| T::default()),
            ),
            Self::Custom(f) => f.as_mut()(samples),
        }
    }
}

impl<T: Div<Output = T> + std::iter::Sum + Default + Debug, S: Sample<Value = T>> Debug
    for ResamplingFunction<T, S>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Average => write!(f, "Average"),
            Self::Sum => write!(f, "Sum"),
            Self::Max => write!(f, "Max"),
            Self::Min => write!(f, "Min"),
            Self::First => write!(f, "First"),
            Self::Last => write!(f, "Last"),
            Self::Coalesce => write!(f, "Coalesce"),
            Self::Count => write!(f, "Count"),
            Self::Custom(_) => write!(f, "Custom"),
        }
    }
}

/// The Resampler struct is used to resample a time series of samples. It stores
/// the samples in a buffer and resamples the samples in the buffer when the
/// resample method is called. A resampler can be configured with a resampling
/// function and a resampling interval.
#[derive(Debug, Default)]
pub struct Resampler<
    T: Div<Output = T> + std::iter::Sum + PartialOrd + FromPrimitive + Default + Debug,
    S: Sample<Value = T>,
> {
    /// The time step between each resampled sample
    interval: TimeDelta,
    /// The resampling functions to use for each channel
    resampling_function: ResamplingFunction<T, S>,
    /// The buffer that stores the samples
    buffer: Vec<S>,
    /// Resample the data in the buffer that is not older than max_age_in_intervals. Number of
    /// intervals. If set to 0, all samples are skipped.
    max_age_in_intervals: i32,
    /// The start time of the resampling.
    start: DateTime<Utc>,
    /// The timestamp of the first sample in the buffer. If None, the timestamp
    /// of the first sample in the buffer is used as input_start
    input_start: Option<DateTime<Utc>>,
    /// The interval between the first and the second sample in the buffer
    input_interval: Option<TimeDelta>,
    /// Whether the resampled timestamp should be the first timestamp (if
    /// `first_timestamp` is `true`) or the last timestamp (if
    /// `first_timestamp` is `false`) in the buffer.
    /// If `first_timestamp` is `true`, the resampled timestamp will be the
    /// timestamp of the first sample in the buffer and the aggregation will
    /// be done with the samples that are `interval` in the future.
    /// If `first_timestamp` is `false`, the resampled timestamp will be the
    /// timestamp of the last sample in the buffer and the aggregation will
    /// be done with the samples that are `interval` in the past.
    first_timestamp: bool,
}

impl<
        T: Div<Output = T> + std::iter::Sum + PartialOrd + FromPrimitive + Default + Debug,
        S: Sample<Value = T>,
    > Resampler<T, S>
{
    /// Creates a new Resampler with the given resampling interval and
    /// resampling function.
    pub fn new(
        interval: TimeDelta,
        resampling_function: ResamplingFunction<T, S>,
        max_age_in_intervals: i32,
        start: DateTime<Utc>,
        first_timestamp: bool,
    ) -> Self {
        let aligned_start = epoch_align(interval, start, None);
        Self {
            interval,
            resampling_function,
            max_age_in_intervals,
            start: aligned_start,
            first_timestamp,
            ..Default::default()
        }
    }

    /// Adds a sample to the buffer.
    pub fn push(&mut self, sample: S) {
        self.buffer.push(sample);
    }

    /// Returns a reference to the buffer.
    pub fn buffer(&self) -> &Vec<S> {
        &self.buffer
    }

    /// Resamples the samples in the buffer and returns the resampled samples
    /// until the given end time.
    pub fn resample(&mut self, end: DateTime<Utc>) -> Vec<S> {
        if self.start >= end {
            warn!("start time is greater or equal to end time");
            return vec![];
        }
        let mut res = vec![];
        let mut interval_buffer = vec![];
        let mut buffer_iter = self.buffer.iter();
        let mut next_sample: Option<&S> = buffer_iter.next();
        self.input_start = next_sample.map(|s| s.timestamp());
        let offset = if self.first_timestamp {
            TimeDelta::zero()
        } else {
            self.interval
        };

        // loop over the intervals
        while self.start < end {
            // loop over the samples in the buffer
            while next_sample
                .map(|s| {
                    is_left_of_buffer_edge(
                        self.first_timestamp,
                        &s.timestamp(),
                        &(self.start + self.interval),
                    )
                })
                .unwrap_or(false)
            {
                // next sample is not newer than the current interval
                if let Some(s) = next_sample {
                    // add the sample to the interval_buffer
                    interval_buffer.push(s);
                    // get the next sample
                    next_sample = buffer_iter.next();
                    // update the input_start and input_interval to adapt
                    // the resampling interval to the input data
                    if let Some(input_start) = self.input_start {
                        if self.input_interval.is_none() {
                            self.input_interval =
                                Some((s.timestamp() - input_start).max(self.interval));
                        }
                    }
                }
            }

            // Remove samples from interval_buffer that are older than
            // max_age
            let input_interval = self.input_interval.unwrap_or(self.interval);
            let drain_end_date =
                self.start + self.interval - input_interval * self.max_age_in_intervals;
            interval_buffer.retain(|s| {
                is_right_of_buffer_edge(self.first_timestamp, &s.timestamp(), &drain_end_date)
            });

            // resample the interval_buffer
            res.push(Sample::new(
                self.start + offset,
                self.resampling_function.apply(interval_buffer.as_slice()),
            ));

            // Go to the next interval
            self.start += self.interval;
        }

        // Remove samples from buffer that are older than max_age
        let interval = self.input_interval.unwrap_or(self.interval);
        let drain_end_date = end - interval * self.max_age_in_intervals;
        self.buffer.retain(|s| {
            is_right_of_buffer_edge(self.first_timestamp, &s.timestamp(), &drain_end_date)
        });

        res
    }

    /// Resamples the samples in the buffer and returns the resampled samples
    /// until now.
    pub fn resample_now(&mut self) -> Vec<S> {
        self.resample(Utc::now())
    }
}

impl<
        T: Div<Output = T> + std::iter::Sum + PartialOrd + FromPrimitive + Default + Debug,
        S: Sample<Value = T>,
    > Extend<S> for Resampler<T, S>
{
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.buffer.extend(iter);
    }
}

/// Aligns a timestamp to the epoch of the resampling interval.
pub(crate) fn epoch_align(
    interval: TimeDelta,
    timestamp: DateTime<Utc>,
    alignment_timestamp: Option<DateTime<Utc>>,
) -> DateTime<Utc> {
    let alignment_timestamp = alignment_timestamp.unwrap_or(DateTime::UNIX_EPOCH);
    DateTime::from_timestamp_millis(
        (timestamp.timestamp_millis() / interval.num_milliseconds()) * interval.num_milliseconds()
            + alignment_timestamp.timestamp_millis(),
    )
    .unwrap_or(timestamp)
}

fn is_left_of_buffer_edge(
    first_timestamp: bool,
    timestamp: &DateTime<Utc>,
    edge_timestamp: &DateTime<Utc>,
) -> bool {
    if first_timestamp {
        timestamp < edge_timestamp
    } else {
        timestamp <= edge_timestamp
    }
}

fn is_right_of_buffer_edge(
    first_timestamp: bool,
    timestamp: &DateTime<Utc>,
    edge_timestamp: &DateTime<Utc>,
) -> bool {
    if first_timestamp {
        timestamp >= edge_timestamp
    } else {
        timestamp > edge_timestamp
    }
}
