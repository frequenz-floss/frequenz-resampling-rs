# Frequenz Resampling Release Notes

## Breaking Changes

- **API Refactoring**: Replaced the single boolean `first_timestamp` parameter with two explicit parameters in `Resampler` and `resample()`:
  - `closed` (`Closed` enum in Rust, `"left"` or `"right"` string in Python): Controls which interval edge is closed for sample membership.
  - `label` (`Label` enum in Rust, `"left"` or `"right"` string in Python): Controls which interval edge is used for output timestamps.
  - This improves API clarity by explicitly separating interval membership semantics from output timestamp labeling.

## Bug Fixes

- Fixed the interval timestamp labeling option to only affect output timestamp labeling, not interval grouping. Previously, using the right-edge label would shift interval boundaries, causing the first sample at `t=0` to be excluded. Now intervals are consistently `[start, end)` regardless of which label is used.

## New Features

- `ResamplingFunction` now implements `Clone`.
- Added `resample()` function for one-shot resampling without managing a `Resampler` instance. Available in both Rust and Python.
- New `Closed` and `Label` enums provide explicit control over interval semantics and output timestamp labeling.
