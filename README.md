# Frequenz Resampling

[<img alt="docs.rs" src="https://img.shields.io/docsrs/frequenz-resampling">](https://docs.rs/frequenz-resampling)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/frequenz-resampling">](https://crates.io/crates/frequenz-resampling)

This project is the rust resampler for resampling a stream of samples to a given interval.
For the Python package, see the
[`frequenz-resampling-python`](https://github.com/frequenz-floss/frequenz-resampling-python)
repository.

## Usage

### One-Shot Resampling

For simple use cases where you want to resample data in a single call, use the `resample()` function:

```rust
use chrono::{DateTime, TimeDelta, Utc};
use frequenz_resampling::{resample, Closed, Label, ResamplingFunction, SimpleSample};

let start = DateTime::from_timestamp(0, 0).unwrap();
let step = TimeDelta::seconds(1);
let data: Vec<(DateTime<Utc>, Option<f64>)> = (0..10)
    .map(|i| (start + step * i, Some((i + 1) as f64)))
    .collect();

let result = resample(
    &data,
    TimeDelta::seconds(5),
    ResamplingFunction::Average,
    Closed::Left,
    Label::Left,
);
// Result: [(t=0, 3.0), (t=5, 8.0)]
```

### Stateful Resampling

For streaming use cases where you need to push samples over time, use the `Resampler` struct.
The construction of a resampler expects an interval (`TimeDelta`) and a
`ResamplingFunction`.
Moreover, the `max_age_in_intervals` parameter can be used to control the maximum age of a sample.
If set to 0, all samples are skipped.
The `start` parameter is used to set the start time of the first resampled sample.

```rust
use chrono::{DateTime, TimeDelta};
use frequenz_resampling::{Closed, Label, Resampler, ResamplingFunction, Sample};

let start = DateTime::from_timestamp(0, 0).unwrap();
let mut resampler: Resampler<f64, TestSample> =
    Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        1,
        start,
        Closed::Left,
        Label::Right,
    );
let step = TimeDelta::seconds(1);
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

resampler.extend(&data);

let resampled = resampler.resample(start + step * 10);

let expected = vec![
    TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
    TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(8.0)),
];

assert_eq!(resampled, expected);
```
