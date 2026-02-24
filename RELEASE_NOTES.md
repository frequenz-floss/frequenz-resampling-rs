# Frequenz Resampling Release Notes

## Bug Fixes

- Fixed `first_timestamp` parameter to only affect output timestamp labeling, not interval grouping. Previously, setting `first_timestamp=false` would shift interval boundaries, causing the first sample at `t=0` to be excluded. Now intervals are consistently `[start, end)` regardless of `first_timestamp` value.

## New Features

- `ResamplingFunction` now implements `Clone`.
