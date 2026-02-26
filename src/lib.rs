// License: MIT
// Copyright © 2024 Frequenz Energy-as-a-Service GmbH

/*!
# frequenz-resampling-rs

This project is the rust resampler for resampling a stream of samples to a
given interval.

## Usage

An instance of the [`Resampler`] can be created with the
[`new`][Resampler::new] method.
Raw data can be added to the resampler either through the
[`push`][Resampler::push] or [`extend`][Resampler::extend] methods, and the
[`resample`][Resampler::resample] method resamples the data that was added to
the buffer.

```rust
use chrono::{DateTime, TimeDelta, Utc};
use frequenz_resampling::{Resampler, ResamplingFunction, Sample};

#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub(crate) struct TestSample {
    timestamp: DateTime<Utc>,
    value: Option<f64>,
}

impl Sample for TestSample {
    type Value = f64;

    fn new(timestamp: DateTime<Utc>, value: Option<f64>) -> Self {
        Self { timestamp, value }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn value(&self) -> Option<f64> {
        self.value
    }
}

let start = DateTime::from_timestamp(0, 0).unwrap();
let mut resampler: Resampler<f64, TestSample> =
    Resampler::new(TimeDelta::seconds(5), ResamplingFunction::Average, 1, start, false);

let step = TimeDelta::seconds(1);
// Data starts at t=0 with values 1-10
// Interval [0, 5): t=0,1,2,3,4 with values 1,2,3,4,5 → avg = 3.0
// Interval [5, 10): t=5,6,7,8,9 with values 6,7,8,9,10 → avg = 8.0
let data = vec![
    TestSample::new(start, Some(1.0)),
    TestSample::new(start + step, Some(2.0)),
    TestSample::new(start + step * 2, Some(3.0)),
    TestSample::new(start + step * 3, Some(4.0)),
    TestSample::new(start + step * 4, Some(5.0)),
    TestSample::new(start + step * 5, Some(6.0)),
    TestSample::new(start + step * 6, Some(7.0)),
    TestSample::new(start + step * 7, Some(8.0)),
    TestSample::new(start + step * 8, Some(9.0)),
    TestSample::new(start + step * 9, Some(10.0)),
];

resampler.extend(data);

let resampled = resampler.resample(start + step * 10);

let expected = vec![
    TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
    TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(8.0)),
];

assert_eq!(resampled, expected);
```
*/

mod resampler;

#[cfg(test)]
mod tests;

#[cfg(feature = "python")]
mod python;

mod resampling_function;
pub use resampling_function::ResamplingFunction;

pub use resampler::{epoch_align, Resampler, Sample};

use chrono::{DateTime, TimeDelta, Utc};

/// A simple sample type for use with the `resample` function.
#[derive(Default, Clone, Debug, Copy, PartialEq)]
pub struct SimpleSample {
    timestamp: DateTime<Utc>,
    value: Option<f64>,
}

impl Sample for SimpleSample {
    type Value = f64;

    fn new(timestamp: DateTime<Utc>, value: Option<f64>) -> Self {
        Self { timestamp, value }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn value(&self) -> Option<f64> {
        self.value
    }
}

/// Resamples a list of timestamp/value pairs in a single call.
///
/// This is a convenience function for one-shot resampling without needing to
/// manage a `Resampler` instance.
///
/// # Arguments
///
/// * `data` - A slice of (timestamp, value) tuples to resample. Must be sorted by timestamp.
/// * `interval` - The resampling interval.
/// * `resampling_function` - The function to use for aggregating values within each interval.
/// * `first_timestamp` - If `true`, output timestamps are set to the start of each interval.
///   If `false`, output timestamps are set to the end of each interval.
///
/// # Returns
///
/// A vector of (timestamp, value) tuples representing the resampled data.
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, TimeDelta, Utc};
/// use frequenz_resampling::{resample, ResamplingFunction, SimpleSample};
///
/// let start = DateTime::from_timestamp(0, 0).unwrap();
/// let step = TimeDelta::seconds(1);
/// let data: Vec<(DateTime<Utc>, Option<f64>)> = (0..10)
///     .map(|i| (start + step * i, Some((i + 1) as f64)))
///     .collect();
///
/// let result = resample(&data, TimeDelta::seconds(5), ResamplingFunction::Average, true);
/// // Result: [(t=0, 3.0), (t=5, 8.0)]
/// assert_eq!(result.len(), 2);
/// assert_eq!(result[0].1, Some(3.0));
/// assert_eq!(result[1].1, Some(8.0));
/// ```
pub fn resample(
    data: &[(DateTime<Utc>, Option<f64>)],
    interval: TimeDelta,
    resampling_function: ResamplingFunction<f64, SimpleSample>,
    first_timestamp: bool,
) -> Vec<(DateTime<Utc>, Option<f64>)> {
    let (Some(first_ts), Some(last_ts)) = (
        data.first().map(|(ts, _)| *ts),
        data.last().map(|(ts, _)| *ts),
    ) else {
        return vec![];
    };

    let aligned_start = epoch_align(interval, first_ts, None);
    let end = epoch_align(interval, last_ts, None) + interval;

    let mut resampler: Resampler<f64, SimpleSample> = Resampler::new(
        interval,
        resampling_function,
        1,
        aligned_start,
        first_timestamp,
    );
    resampler.extend(data.iter().map(|(ts, val)| SimpleSample::new(*ts, *val)));
    resampler
        .resample(end)
        .into_iter()
        .map(|s| (s.timestamp(), s.value()))
        .collect()
}
