# Frequenz Resampling Release Notes

## Summary

## Upgrading

## New Features

- Adds a resampler for a stream of samples to resample to a given interval.
- Adds python bindings for the resampler.
- Adapts the ResamplingFunction Python interface to Python Enums.
- Makes custom resampling function Sync.
- Adds a `first_timestamp` parameter to the resampler to control whether the
  resampled timestamp should be the first timestamp in the buffer or the last
  timestamp in the buffer.
- Adds `first` resampling function.
- Adds `coalesce` resampling function.

## Bug Fixes
