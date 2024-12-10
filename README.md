# Frequenz Resampling

This project is the rust resampler for resampling a stream of samples to a given interval.

## Usage in Rust

To resample a vector of samples to a given interval, you can use the `Resampler` struct.
The construction of a resampler expects an interval (`TimeDelta`) and a
`ResamplingFunction`.
Moreover, the `max_age_in_intervals` parameter can be used to control the maximum age of a sample.
If set to 0, all samples are skipped.
The `start` parameter is used to set the start time of the first resampled sample.

```rust
use chrono::{DateTime, TimeDelta};
use frequenz_resampling::{Resampler, ResamplingFunction, Sample};

let start = DateTime::from_timestamp(0, 0).unwrap();
let mut resampler: Resampler<f64, TestSample> =
    Resampler::new(TimeDelta::seconds(5), ResamplingFunction::Average, 1, start, false);
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


## Usage in Python

To resample a stream of samples to a given interval, you can use the `Resampler`
class.
The construction of a resampler expects an interval (`datetime.timedelta`),
a `ResamplingFunction`, a `max_age_in_intervals` parameter to control the
maximum age of a sample (skips all samples if set to `0`), and a `start` parameter to set the start time of the
first resampled sample.

```python
import datetime as dt
from frequenz.resampling import Resampler, ResamplingFunction


start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
step = dt.timedelta(seconds=1)
resampler = Resampler(
    dt.timedelta(seconds=5),
    ResamplingFunction.Average,
    max_age_in_intervals=1,
    start=start,
    first_timestamp=False,
)

for i in range(10):
    resampler.push_sample(timestamp=start + i * step, value=i + 1)

expected = [
    (start + 5 * step, 3.0),
    (start + 10 * step, 8.0),
]

resampled = resampler.resample(start + 10 * step)

assert resampled == expected
```
