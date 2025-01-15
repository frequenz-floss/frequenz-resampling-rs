// License: MIT
// Copyright © 2024 Frequenz Energy-as-a-Service GmbH

//! This file contains tests for the resampler module.

use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, Div},
};

use crate::resampler::{epoch_align, Resampler, ResamplingFunction, Sample};
use chrono::{DateTime, TimeDelta, Utc};
use num_traits::FromPrimitive;

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

fn test_resampling(
    resampling_function: ResamplingFunction<f64, TestSample>,
    expected: Vec<TestSample>,
) {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> =
        Resampler::new(TimeDelta::seconds(5), resampling_function, 1, start, false);
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, Some(1.0)),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 3, Some(3.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 6, Some(6.0)),
        TestSample::new(start + step * 7, Some(7.0)),
        TestSample::new(start + step * 8, Some(8.0)),
        TestSample::new(start + step * 9, Some(9.0)),
        TestSample::new(start + step * 10, Some(10.0)),
    ];

    resampler.extend(data);

    let resampled = resampler.resample(start + step * 10);
    assert_eq!(resampled, expected);
}

fn test_resampling_with_none_first(
    resampling_function: ResamplingFunction<f64, TestSample>,
    expected: Vec<TestSample>,
) {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> =
        Resampler::new(TimeDelta::seconds(5), resampling_function, 1, start, false);
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, None),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 3, Some(3.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 6, None),
        TestSample::new(start + step * 7, Some(7.0)),
        TestSample::new(start + step * 8, Some(8.0)),
        TestSample::new(start + step * 9, Some(9.0)),
        TestSample::new(start + step * 10, Some(10.0)),
    ];

    resampler.extend(data);

    let resampled = resampler.resample(start + step * 10);
    assert_eq!(resampled, expected);
}

fn test_resampling_with_none_all(
    resampling_function: ResamplingFunction<f64, TestSample>,
    expected: Vec<TestSample>,
) {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> =
        Resampler::new(TimeDelta::seconds(5), resampling_function, 1, start, false);
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, None),
        TestSample::new(start + step * 2, None),
        TestSample::new(start + step * 3, None),
        TestSample::new(start + step * 4, None),
        TestSample::new(start + step * 5, None),
        TestSample::new(start + step * 6, None),
        TestSample::new(start + step * 7, None),
        TestSample::new(start + step * 8, None),
        TestSample::new(start + step * 9, None),
        TestSample::new(start + step * 10, None),
    ];

    resampler.extend(data);

    let resampled = resampler.resample(start + step * 10);
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_average() {
    test_resampling(
        ResamplingFunction::Average,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(8.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Average,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.5)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(8.5)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Average,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_count() {
    test_resampling(
        ResamplingFunction::Count,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(5.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(5.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Count,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(4.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(4.0)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Count,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(0.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(0.0)),
        ],
    );
}

#[test]
fn test_resampling_sum() {
    test_resampling(
        ResamplingFunction::Sum,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(15.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(40.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Sum,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(14.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(34.0)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Sum,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_min() {
    test_resampling(
        ResamplingFunction::Min,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(1.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(6.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Min,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(2.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(7.0)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Min,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_max() {
    test_resampling(
        ResamplingFunction::Max,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(5.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(10.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Max,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(5.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(10.0)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Max,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_first() {
    test_resampling(
        ResamplingFunction::First,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(1.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(6.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::First,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::First,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_last() {
    test_resampling(
        ResamplingFunction::Last,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(5.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(10.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Last,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(5.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(10.0)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Last,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_coalesce() {
    test_resampling(
        ResamplingFunction::Coalesce,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(1.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(6.0)),
        ],
    );

    test_resampling_with_none_first(
        ResamplingFunction::Coalesce,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(2.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(7.0)),
        ],
    );

    test_resampling_with_none_all(
        ResamplingFunction::Coalesce,
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        ],
    );
}

#[test]
fn test_resampling_custom() {
    test_resampling(
        ResamplingFunction::Custom(Box::new(|x: &[&TestSample]| {
            Some(x.iter().map(|s| s.value().unwrap()).sum::<f64>())
        })),
        vec![
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(15.0)),
            TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(40.0)),
        ],
    );
}

#[test]
fn test_resampling_with_max_age() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        2,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, Some(1.0)),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 3, Some(3.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 6, Some(6.0)),
        TestSample::new(start + step * 7, Some(7.0)),
        TestSample::new(start + step * 8, Some(8.0)),
        TestSample::new(start + step * 9, Some(9.0)),
        TestSample::new(start + step * 10, Some(10.0)),
        TestSample::new(start + step * 11, Some(11.0)),
        TestSample::new(start + step * 12, Some(12.0)),
        TestSample::new(start + step * 13, Some(13.0)),
        TestSample::new(start + step * 14, Some(14.0)),
        TestSample::new(start + step * 15, Some(15.0)),
    ];

    resampler.extend(data);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(5.5)),
        TestSample::new(DateTime::from_timestamp(15, 0).unwrap(), Some(10.5)),
    ];

    let resampled = resampler.resample(start + step * 15);
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_with_zero_max_age() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        0,
        start,
        false,
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
        TestSample::new(start + step * 10, Some(11.0)),
        TestSample::new(start + step * 11, Some(12.0)),
        TestSample::new(start + step * 12, Some(13.0)),
        TestSample::new(start + step * 13, Some(14.0)),
        TestSample::new(start + step * 14, Some(15.0)),
    ];

    resampler.extend(data);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), None),
        TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        TestSample::new(DateTime::from_timestamp(15, 0).unwrap(), None),
    ];

    let resampled = resampler.resample(start + step * 15);
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_with_max_age_older() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        3,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, Some(1.0)),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 3, Some(3.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 6, Some(6.0)),
        TestSample::new(start + step * 7, Some(7.0)),
        TestSample::new(start + step * 8, Some(8.0)),
        TestSample::new(start + step * 9, Some(9.0)),
        TestSample::new(start + step * 10, Some(10.0)),
        TestSample::new(start + step * 11, Some(11.0)),
        TestSample::new(start + step * 12, Some(12.0)),
        TestSample::new(start + step * 13, Some(13.0)),
        TestSample::new(start + step * 14, Some(14.0)),
        TestSample::new(start + step * 15, Some(15.0)),
    ];

    resampler.extend(data);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(5.5)),
        TestSample::new(DateTime::from_timestamp(15, 0).unwrap(), Some(8.0)),
    ];

    let resampled = resampler.resample(start + step * 15);
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_with_max_age_batch() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        2,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data1 = vec![
        TestSample::new(start + step * 1, Some(1.0)),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 3, Some(3.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 6, Some(6.0)),
        TestSample::new(start + step * 7, Some(7.0)),
        TestSample::new(start + step * 8, Some(8.0)),
        TestSample::new(start + step * 9, Some(9.0)),
        TestSample::new(start + step * 10, Some(10.0)),
    ];
    let data2 = vec![
        TestSample::new(start + step * 11, Some(11.0)),
        TestSample::new(start + step * 12, Some(12.0)),
        TestSample::new(start + step * 13, Some(13.0)),
        TestSample::new(start + step * 14, Some(14.0)),
        TestSample::new(start + step * 15, Some(15.0)),
    ];

    resampler.extend(data1);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), Some(5.5)),
    ];

    let resampled = resampler.resample(start + step * 10);
    assert_eq!(resampled, expected);

    resampler.extend(data2);

    let expected2 = vec![TestSample::new(
        DateTime::from_timestamp(15, 0).unwrap(),
        Some(10.5),
    )];

    let resampled2 = resampler.resample(start + step * 15);
    assert_eq!(resampled2, expected2);
}

#[test]
fn test_resampling_with_gap() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        1,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, Some(1.0)),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 17, Some(6.0)),
        TestSample::new(start + step * 20, Some(10.0)),
    ];

    resampler.extend(data);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
        TestSample::new(DateTime::from_timestamp(15, 0).unwrap(), None),
        TestSample::new(DateTime::from_timestamp(20, 0).unwrap(), Some(8.0)),
    ];

    let resampled = resampler.resample(start + step * 20);
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_with_slow_data() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(1),
        ResamplingFunction::Average,
        2,
        start,
        false,
    );
    let offset = TimeDelta::milliseconds(500);
    let step = TimeDelta::seconds(2);
    let data = vec![
        TestSample::new(start + step * 1 - offset, Some(3.0)),
        TestSample::new(start + step * 2 - offset, Some(4.0)),
        TestSample::new(start + step * 3 - offset, Some(5.0)),
        TestSample::new(start + step * 4 - offset, Some(6.0)),
    ];

    resampler.extend(data);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(1, 0).unwrap(), None),
        TestSample::new(DateTime::from_timestamp(2, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(3, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(4, 0).unwrap(), Some(4.0)),
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(4.0)),
        TestSample::new(DateTime::from_timestamp(6, 0).unwrap(), Some(5.0)),
        TestSample::new(DateTime::from_timestamp(7, 0).unwrap(), Some(5.0)),
        TestSample::new(DateTime::from_timestamp(8, 0).unwrap(), Some(6.0)),
    ];

    let resampled = resampler.resample(start + step * 4);
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_with_gap_early_end_date() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        1,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start + step, Some(1.0)),
        TestSample::new(start + step * 2, Some(2.0)),
        TestSample::new(start + step * 4, Some(4.0)),
        TestSample::new(start + step * 5, Some(5.0)),
        TestSample::new(start + step * 17, Some(6.0)),
        TestSample::new(start + step * 20, Some(10.0)),
    ];

    resampler.extend(data);

    let expected = vec![
        TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(3.0)),
        TestSample::new(DateTime::from_timestamp(10, 0).unwrap(), None),
    ];

    let resampled = resampler.resample(start + step * 10);
    assert_eq!(resampled, expected);

    let expected2 = vec![
        TestSample::new(DateTime::from_timestamp(15, 0).unwrap(), None),
        TestSample::new(DateTime::from_timestamp(20, 0).unwrap(), Some(8.0)),
    ];

    let resampled2 = resampler.resample(start + step * 20);
    assert_eq!(resampled2, expected2);
}

#[test]
fn test_empty_buffer() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        1,
        start,
        false,
    );

    let resampled = resampler.resample(start + TimeDelta::seconds(10));
    assert_eq!(
        resampled,
        vec![
            TestSample::new(start + TimeDelta::seconds(5), None),
            TestSample::new(start + TimeDelta::seconds(10), None),
        ]
    );
}

#[test]
fn test_epoch_alignment() {
    let interval = TimeDelta::seconds(5);
    let test_time = DateTime::from_timestamp(3, 0).unwrap();
    assert_eq!(
        epoch_align(interval, test_time, None),
        DateTime::from_timestamp(0, 0).unwrap()
    );
    assert_eq!(
        epoch_align(
            interval,
            test_time,
            Some(DateTime::from_timestamp(1, 0).unwrap())
        ),
        DateTime::from_timestamp(1, 0).unwrap()
    );
}

#[test]
fn test_is_right_of_buffer_edge() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<f64, TestSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        1,
        start,
        true,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        TestSample::new(start, Some(1.0)),
        TestSample::new(start + step * 1, Some(2.0)),
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
    assert_eq!(
        resampled,
        vec![
            TestSample::new(DateTime::from_timestamp(0, 0).unwrap(), Some(3.0)),
            TestSample::new(DateTime::from_timestamp(5, 0).unwrap(), Some(8.0)),
        ],
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct NonPrimitive {
    value: Vec<i32>,
}

impl Add for NonPrimitive {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.into_iter().chain(rhs.value).collect(),
        }
    }
}

impl Div for NonPrimitive {
    type Output = Self;

    fn div(self, _rhs: Self) -> Self::Output {
        Self { value: vec![] }
    }
}

impl Sum for NonPrimitive {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|s| s.value).flatten().collect(),
        }
    }
}

impl PartialOrd for NonPrimitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl FromPrimitive for NonPrimitive {
    fn from_i32(n: i32) -> Option<Self> {
        Some(Self { value: vec![n] })
    }

    fn from_u32(n: u32) -> Option<Self> {
        Some(Self {
            value: vec![n as i32],
        })
    }

    fn from_i64(n: i64) -> Option<Self> {
        Some(Self {
            value: vec![n as i32],
        })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Self {
            value: vec![n as i32],
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct NonPrimitiveSample {
    timestamp: DateTime<Utc>,
    value: Option<NonPrimitive>,
}

impl Sample for NonPrimitiveSample {
    type Value = NonPrimitive;

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn value(&self) -> Option<NonPrimitive> {
        self.value.clone()
    }

    fn new(timestamp: DateTime<Utc>, value: Option<Self::Value>) -> Self {
        Self { timestamp, value }
    }
}

#[test]
fn test_resampling_non_primitive_average() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<NonPrimitive, NonPrimitiveSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Average,
        1,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        NonPrimitiveSample::new(start + step, Some(NonPrimitive { value: vec![1] })),
        NonPrimitiveSample::new(start + step * 2, Some(NonPrimitive { value: vec![2] })),
        NonPrimitiveSample::new(start + step * 3, Some(NonPrimitive { value: vec![3] })),
        NonPrimitiveSample::new(start + step * 4, Some(NonPrimitive { value: vec![4] })),
        NonPrimitiveSample::new(start + step * 5, Some(NonPrimitive { value: vec![5] })),
        NonPrimitiveSample::new(start + step * 6, Some(NonPrimitive { value: vec![6] })),
        NonPrimitiveSample::new(start + step * 7, Some(NonPrimitive { value: vec![7] })),
        NonPrimitiveSample::new(start + step * 8, Some(NonPrimitive { value: vec![8] })),
        NonPrimitiveSample::new(start + step * 9, Some(NonPrimitive { value: vec![9] })),
        NonPrimitiveSample::new(start + step * 10, Some(NonPrimitive { value: vec![10] })),
    ];

    resampler.extend(data);

    let resampled = resampler.resample(start + step * 10);

    let expected = vec![
        NonPrimitiveSample::new(
            DateTime::from_timestamp(5, 0).unwrap(),
            Some(NonPrimitive { value: vec![] }),
        ),
        NonPrimitiveSample::new(
            DateTime::from_timestamp(10, 0).unwrap(),
            Some(NonPrimitive { value: vec![] }),
        ),
    ];
    assert_eq!(resampled, expected);
}

#[test]
fn test_resampling_non_primitive_sum() {
    let start = DateTime::from_timestamp(0, 0).unwrap();
    let mut resampler: Resampler<NonPrimitive, NonPrimitiveSample> = Resampler::new(
        TimeDelta::seconds(5),
        ResamplingFunction::Sum,
        1,
        start,
        false,
    );
    let step = TimeDelta::seconds(1);
    let data = vec![
        NonPrimitiveSample::new(start + step, Some(NonPrimitive { value: vec![1] })),
        NonPrimitiveSample::new(start + step * 2, Some(NonPrimitive { value: vec![2] })),
        NonPrimitiveSample::new(start + step * 3, Some(NonPrimitive { value: vec![3] })),
        NonPrimitiveSample::new(start + step * 4, Some(NonPrimitive { value: vec![4] })),
        NonPrimitiveSample::new(start + step * 5, Some(NonPrimitive { value: vec![5] })),
        NonPrimitiveSample::new(start + step * 6, Some(NonPrimitive { value: vec![6] })),
        NonPrimitiveSample::new(start + step * 7, Some(NonPrimitive { value: vec![7] })),
        NonPrimitiveSample::new(start + step * 8, Some(NonPrimitive { value: vec![8] })),
        NonPrimitiveSample::new(start + step * 9, Some(NonPrimitive { value: vec![9] })),
        NonPrimitiveSample::new(start + step * 10, Some(NonPrimitive { value: vec![10] })),
    ];

    resampler.extend(data);

    let resampled = resampler.resample(start + step * 10);

    let expected = vec![
        NonPrimitiveSample::new(
            DateTime::from_timestamp(5, 0).unwrap(),
            Some(NonPrimitive {
                value: vec![1, 2, 3, 4, 5],
            }),
        ),
        NonPrimitiveSample::new(
            DateTime::from_timestamp(10, 0).unwrap(),
            Some(NonPrimitive {
                value: vec![6, 7, 8, 9, 10],
            }),
        ),
    ];
    assert_eq!(resampled, expected);
}
