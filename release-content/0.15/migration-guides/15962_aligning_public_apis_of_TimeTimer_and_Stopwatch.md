The APIs of `Time`, `Timer` and `Stopwatch` have been cleaned up for consistency with each other and the standard library’s `Duration` type. The following methods have been renamed:

- `Stowatch::paused` -> `Stopwatch::is_paused`
- `Time::elapsed_seconds` -> `Time::elasped_secs` (including `_f64` and `_wrapped` variants)
