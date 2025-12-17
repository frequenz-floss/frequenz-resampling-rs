// License: MIT
// Copyright © 2024 Frequenz Energy-as-a-Service GmbH

//! Resampling functions for time series data.

use crate::Sample;
use std::{fmt::Debug, ops::Div};

pub type CustomResamplingFunction<S, T> = Box<dyn FnMut(&[&S]) -> Option<T> + Send + Sync>;

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
