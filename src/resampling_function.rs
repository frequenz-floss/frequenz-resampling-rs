// License: MIT
// Copyright © 2024 Frequenz Energy-as-a-Service GmbH

//! Resampling functions for time series data.

use crate::Sample;
use std::{fmt::Debug, ops::Div};

// This trait can't directly have `Clone` as a supertrait because
// then it wouldn't be dyn-compatible.
//
// Instead, we make a blanket implementation for all FnMut that implement
// `Clone`, which will be dispatched statically.
pub trait ClonableFnMut<S, T>: FnMut(&[&S]) -> Option<T> + Send + Sync {
    fn clone_box(&self) -> Box<dyn ClonableFnMut<S, T>>;
}

impl<S, T, F> ClonableFnMut<S, T> for F
where
    F: FnMut(&[&S]) -> Option<T> + Send + Sync + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn ClonableFnMut<S, T>> {
        Box::new(self.clone())
    }
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
    Custom(Box<dyn ClonableFnMut<S, T>>),
}

impl<T, S> Clone for ResamplingFunction<T, S>
where
    T: Div<Output = T> + std::iter::Sum + Default + Debug,
    S: Sample<Value = T>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Average => Self::Average,
            Self::Sum => Self::Sum,
            Self::Max => Self::Max,
            Self::Min => Self::Min,
            Self::First => Self::First,
            Self::Last => Self::Last,
            Self::Coalesce => Self::Coalesce,
            Self::Count => Self::Count,
            Self::Custom(f) => Self::Custom(f.clone_box()),
        }
    }
}
