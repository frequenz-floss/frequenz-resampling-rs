use crate::{resampler::Resampler, ResamplingFunction, Sample};
use chrono::{DateTime, TimeDelta, Utc};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::fmt::Display;

#[derive(Default, Clone, Debug, Copy)]
struct PythonSample {
    timestamp: DateTime<Utc>,
    value: Option<f32>,
}

impl PythonSample {
    fn to_tuple(self) -> (DateTime<Utc>, Option<f32>) {
        (self.timestamp, self.value)
    }
}

impl Sample for PythonSample {
    type Value = f32;

    fn new(timestamp: DateTime<Utc>, value: Option<f32>) -> Self {
        Self { timestamp, value }
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    fn value(&self) -> Option<f32> {
        self.value
    }
}

#[pyclass(eq, eq_int, name = "ResamplingFunction")]
#[derive(Clone, Debug, Copy, PartialEq)]
enum ResamplingFunctionF32 {
    Average,
    Sum,
    Max,
    Min,
    Last,
    Count,
}

#[pymethods]
impl ResamplingFunctionF32 {
    #[new]
    fn new(value: i32) -> PyResult<Self> {
        value.try_into()
    }

    #[staticmethod]
    fn values() -> Vec<i32> {
        vec![
            Self::Average.value(),
            Self::Sum.value(),
            Self::Max.value(),
            Self::Min.value(),
            Self::Last.value(),
            Self::Count.value(),
        ]
    }

    #[staticmethod]
    fn members() -> Vec<(String, i32)> {
        vec![
            (Self::Average.to_string(), Self::Average.value()),
            (Self::Sum.to_string(), Self::Sum.value()),
            (Self::Max.to_string(), Self::Max.value()),
            (Self::Min.to_string(), Self::Min.value()),
            (Self::Last.to_string(), Self::Last.value()),
            (Self::Count.to_string(), Self::Count.value()),
        ]
    }

    fn __str__(&self) -> String {
        format!("{}.{}", "ResamplingFunction", self)
    }

    fn __repr__(&self) -> String {
        format!("<{}: {}>", self.__str__(), self.value())
    }

    #[getter]
    fn name(&self) -> String {
        self.to_string()
    }

    #[getter]
    fn value(&self) -> i32 {
        match self {
            Self::Average => 0,
            Self::Sum => 1,
            Self::Max => 2,
            Self::Min => 3,
            Self::Last => 4,
            Self::Count => 5,
        }
    }
}

impl TryFrom<i32> for ResamplingFunctionF32 {
    type Error = PyErr;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Average),
            1 => Ok(Self::Sum),
            2 => Ok(Self::Max),
            3 => Ok(Self::Min),
            4 => Ok(Self::Last),
            5 => Ok(Self::Count),
            _ => Err(PyValueError::new_err("Invalid resampling function")),
        }
    }
}

impl Display for ResamplingFunctionF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ResamplingFunctionF32::Average => "Average",
                ResamplingFunctionF32::Sum => "Sum",
                ResamplingFunctionF32::Max => "Max",
                ResamplingFunctionF32::Min => "Min",
                ResamplingFunctionF32::Last => "Last",
                ResamplingFunctionF32::Count => "Count",
            }
        )
    }
}

impl From<ResamplingFunctionF32> for ResamplingFunction<f32, PythonSample> {
    fn from(resampling_function: ResamplingFunctionF32) -> Self {
        match resampling_function {
            ResamplingFunctionF32::Average => ResamplingFunction::Average,
            ResamplingFunctionF32::Sum => ResamplingFunction::Sum,
            ResamplingFunctionF32::Max => ResamplingFunction::Max,
            ResamplingFunctionF32::Min => ResamplingFunction::Min,
            ResamplingFunctionF32::Last => ResamplingFunction::Last,
            ResamplingFunctionF32::Count => ResamplingFunction::Count,
        }
    }
}

/// The Resampler class for f32 values.
#[pyclass(name = "Resampler")]
struct ResamplerF32 {
    inner: Resampler<f32, PythonSample>,
}

#[pymethods]
impl ResamplerF32 {
    #[new]
    #[pyo3(signature = (interval, resampling_function, *, max_age_in_intervals, start))]
    fn new(
        interval: TimeDelta,
        resampling_function: ResamplingFunctionF32,
        max_age_in_intervals: i32,
        start: DateTime<Utc>,
    ) -> Self {
        Self {
            inner: Resampler::new(
                interval,
                resampling_function.into(),
                max_age_in_intervals,
                start,
            ),
        }
    }

    #[pyo3(signature = (*, timestamp, value))]
    fn push_sample(&mut self, timestamp: DateTime<Utc>, value: Option<f32>) {
        self.inner.push(PythonSample::new(timestamp, value));
    }

    #[pyo3(signature = (end=None))]
    fn resample(&mut self, end: Option<DateTime<Utc>>) -> Vec<(DateTime<Utc>, Option<f32>)> {
        match end {
            Some(end) => self.inner.resample(end),
            None => self.inner.resample_now(),
        }
        .into_iter()
        .map(PythonSample::to_tuple)
        .collect()
    }
}

#[pymodule]
fn _rust_backend(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ResamplerF32>()?;
    m.add_class::<ResamplingFunctionF32>()?;
    Ok(())
}
