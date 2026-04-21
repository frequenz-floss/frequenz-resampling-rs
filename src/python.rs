use crate::{resampler::Resampler, Closed, Label, ResamplingFunction, Sample};
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
    First,
    Coalesce,
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
            Self::First.value(),
            Self::Coalesce.value(),
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
            (Self::First.to_string(), Self::First.value()),
            (Self::Coalesce.to_string(), Self::Coalesce.value()),
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
            Self::First => 6,
            Self::Coalesce => 7,
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
            6 => Ok(Self::First),
            7 => Ok(Self::Coalesce),
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
                ResamplingFunctionF32::First => "First",
                ResamplingFunctionF32::Last => "Last",
                ResamplingFunctionF32::Coalesce => "Coalesce",
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
            ResamplingFunctionF32::First => ResamplingFunction::First,
            ResamplingFunctionF32::Last => ResamplingFunction::Last,
            ResamplingFunctionF32::Coalesce => ResamplingFunction::Coalesce,
            ResamplingFunctionF32::Count => ResamplingFunction::Count,
        }
    }
}

impl From<ResamplingFunctionF32> for ResamplingFunction<f64, crate::SimpleSample> {
    fn from(resampling_function: ResamplingFunctionF32) -> Self {
        match resampling_function {
            ResamplingFunctionF32::Average => ResamplingFunction::Average,
            ResamplingFunctionF32::Sum => ResamplingFunction::Sum,
            ResamplingFunctionF32::Max => ResamplingFunction::Max,
            ResamplingFunctionF32::Min => ResamplingFunction::Min,
            ResamplingFunctionF32::First => ResamplingFunction::First,
            ResamplingFunctionF32::Last => ResamplingFunction::Last,
            ResamplingFunctionF32::Coalesce => ResamplingFunction::Coalesce,
            ResamplingFunctionF32::Count => ResamplingFunction::Count,
        }
    }
}

#[pyclass(eq, eq_int, name = "Closed")]
#[derive(Clone, Debug, Copy, PartialEq)]
enum ClosedPy {
    Left,
    Right,
}

#[pymethods]
impl ClosedPy {
    #[new]
    fn new(value: i32) -> PyResult<Self> {
        value.try_into()
    }

    #[staticmethod]
    fn values() -> Vec<i32> {
        vec![Self::Left.value(), Self::Right.value()]
    }

    #[staticmethod]
    fn members() -> Vec<(String, i32)> {
        vec![
            (Self::Left.to_string(), Self::Left.value()),
            (Self::Right.to_string(), Self::Right.value()),
        ]
    }

    fn __str__(&self) -> String {
        format!("{}.{}", "Closed", self)
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
            Self::Left => 0,
            Self::Right => 1,
        }
    }
}

impl TryFrom<i32> for ClosedPy {
    type Error = PyErr;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err(PyValueError::new_err("Invalid closed value")),
        }
    }
}

impl Display for ClosedPy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClosedPy::Left => "Left",
                ClosedPy::Right => "Right",
            }
        )
    }
}

impl From<ClosedPy> for Closed {
    fn from(closed: ClosedPy) -> Self {
        match closed {
            ClosedPy::Left => Closed::Left,
            ClosedPy::Right => Closed::Right,
        }
    }
}

#[pyclass(eq, eq_int, name = "Label")]
#[derive(Clone, Debug, Copy, PartialEq)]
enum LabelPy {
    Left,
    Right,
}

#[pymethods]
impl LabelPy {
    #[new]
    fn new(value: i32) -> PyResult<Self> {
        value.try_into()
    }

    #[staticmethod]
    fn values() -> Vec<i32> {
        vec![Self::Left.value(), Self::Right.value()]
    }

    #[staticmethod]
    fn members() -> Vec<(String, i32)> {
        vec![
            (Self::Left.to_string(), Self::Left.value()),
            (Self::Right.to_string(), Self::Right.value()),
        ]
    }

    fn __str__(&self) -> String {
        format!("{}.{}", "Label", self)
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
            Self::Left => 0,
            Self::Right => 1,
        }
    }
}

impl TryFrom<i32> for LabelPy {
    type Error = PyErr;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            _ => Err(PyValueError::new_err("Invalid label")),
        }
    }
}

impl Display for LabelPy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LabelPy::Left => "Left",
                LabelPy::Right => "Right",
            }
        )
    }
}

impl From<LabelPy> for Label {
    fn from(label: LabelPy) -> Self {
        match label {
            LabelPy::Left => Label::Left,
            LabelPy::Right => Label::Right,
        }
    }
}

/// Resamples a list of timestamp/value pairs in a single call.
///
/// This is a convenience function for one-shot resampling without needing to
/// manage a `Resampler` instance.
///
/// Args:
///     data: A list of (timestamp, value) tuples to resample. Must be sorted by timestamp.
///     interval: The resampling interval.
///     method: The resampling function to use for aggregating values within each interval.
///     closed: Which interval edge is closed for sample membership.
///     label: Which interval edge to use for output timestamps.
///
/// Returns:
///     A list of (timestamp, value) tuples representing the resampled data.
#[pyfunction]
#[pyo3(signature = (data, interval, method, *, closed, label))]
fn resample(
    data: Vec<(DateTime<Utc>, Option<f32>)>,
    interval: TimeDelta,
    method: ResamplingFunctionF32,
    closed: ClosedPy,
    label: LabelPy,
) -> PyResult<Vec<(DateTime<Utc>, Option<f32>)>> {
    // Convert f32 to f64 for the Rust implementation
    let data_f64: Vec<(DateTime<Utc>, Option<f64>)> = data
        .into_iter()
        .map(|(ts, val)| (ts, val.map(|v| v as f64)))
        .collect();

    // Call the Rust implementation
    let result = crate::resample(
        &data_f64,
        interval,
        method.into(),
        closed.into(),
        label.into(),
    );

    // Convert f64 back to f32
    Ok(result
        .into_iter()
        .map(|(ts, val)| (ts, val.map(|v| v as f32)))
        .collect())
}

/// The Resampler class for f32 values.
#[pyclass(name = "Resampler")]
struct ResamplerF32 {
    inner: Resampler<f32, PythonSample>,
}

#[pymethods]
impl ResamplerF32 {
    #[new]
    #[pyo3(signature = (interval, resampling_function, *, max_age_in_intervals, start, closed, label))]
    fn new(
        interval: TimeDelta,
        resampling_function: ResamplingFunctionF32,
        max_age_in_intervals: i32,
        start: DateTime<Utc>,
        closed: ClosedPy,
        label: LabelPy,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: Resampler::new(
                interval,
                resampling_function.into(),
                max_age_in_intervals,
                start,
                closed.into(),
                label.into(),
            ),
        })
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
    m.add_class::<ClosedPy>()?;
    m.add_class::<LabelPy>()?;
    m.add_function(wrap_pyfunction!(resample, m)?)?;
    Ok(())
}
