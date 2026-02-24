# Frequenz Resampling

[<img alt="docs.rs" src="https://img.shields.io/docsrs/frequenz-resampling">](https://docs.rs/frequenz-resampling)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/frequenz-resampling">](https://crates.io/crates/frequenz-resampling)
[<img alt="PyPI" src="https://img.shields.io/pypi/v/frequenz-resampling">](https://pypi.org/project/frequenz-resampling/)

A high-performance library for resampling time series data to fixed intervals. Built in Rust with Python bindings via PyO3.

## Features

- Resample irregular time series data to uniform intervals
- Multiple built-in resampling functions: `Average`, `Sum`, `Max`, `Min`, `First`, `Last`, `Coalesce`, `Count`
- Support for custom resampling functions
- Configurable sample age limits to handle stale data
- Available for both Rust and Python

## Installation

### Rust

Add the following to your `Cargo.toml`:

```toml
[dependencies]
frequenz-resampling = "0.2"
```

### Python

```bash
pip install frequenz-resampling
```

Requires Python 3.11 or later.

## Quick Start

### Rust

```rust
use chrono::{DateTime, TimeDelta};
use frequenz_resampling::{Resampler, ResamplingFunction};

// Create a resampler with 5-second intervals using average aggregation
let start = DateTime::from_timestamp(0, 0).unwrap();
let mut resampler = Resampler::new(
    TimeDelta::seconds(5),
    ResamplingFunction::Average,
    1,     // max_age_in_intervals
    start,
    false, // first_timestamp
);

// Push samples and resample
// See full usage example below
```

### Python

```python
import datetime as dt
from frequenz.resampling import Resampler, ResamplingFunction

# Create a resampler with 5-second intervals
start = dt.datetime(1970, 1, 1, tzinfo=dt.timezone.utc)
resampler = Resampler(
    dt.timedelta(seconds=5),
    ResamplingFunction.Average,
    max_age_in_intervals=1,
    start=start,
)

# Push samples
for i in range(10):
    resampler.push_sample(timestamp=start + i * dt.timedelta(seconds=1), value=i + 1)

# Get resampled data
resampled = resampler.resample(start + dt.timedelta(seconds=10))
```

## Documentation

- **Rust**: [docs.rs/frequenz-resampling](https://docs.rs/frequenz-resampling)
- **Python**: API documentation available via the installed package

## Usage

### Rust

To resample a vector of samples to a given interval, use the `Resampler` struct. The constructor accepts:

- `interval`: The target resampling interval (`TimeDelta`)
- `resampling_function`: How to aggregate samples within each interval (`ResamplingFunction`)
- `max_age_in_intervals`: Maximum age of samples to consider (set to 0 to skip all samples)
- `start`: Start time of the first resampled sample
- `first_timestamp`: Whether to use the first timestamp in each interval (default: last timestamp)

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

### Python

The Python API mirrors the Rust interface. Create a `Resampler` with the same parameters:

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

## Resampling Functions

| Function | Description |
|----------|-------------|
| `Average` | Calculates the mean of all samples in the interval |
| `Sum` | Calculates the sum of all samples |
| `Max` | Returns the maximum value |
| `Min` | Returns the minimum value |
| `First` | Returns the first sample value |
| `Last` | Returns the last sample value |
| `Coalesce` | Returns the first non-None sample |
| `Count` | Returns the number of samples |
| `Custom` | User-defined aggregation function (Rust only) |

## License

This project is licensed under the MIT License.
