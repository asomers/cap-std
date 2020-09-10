#[cfg(not(windows))]
use posish::time::{clock_getres, ClockId};
use std::{convert::TryInto, time, time::Duration};
#[cfg(windows)]
use winx::time::perf_counter_frequency;

/// Extension trait for `cap_std::time::MonotonicClock`.
pub trait MonotonicClockExt {
    /// A monotonic clock datapoint.
    type Instant;

    /// Similar to `MonotonicClock::now`, but takes an additional `precision`
    /// parameter allowing callers to inform the implementation when they
    /// don't need full precision. The implementation need not make any
    /// effort to provide a time with greater precision.
    fn now_with(&self, precision: Duration) -> Self::Instant;

    /// Return the resolution of the clock.
    fn resolution(&self) -> Duration;
}

#[cfg(not(windows))]
impl MonotonicClockExt for cap_primitives::time::MonotonicClock {
    type Instant = cap_primitives::time::Instant;

    #[cfg(not(target_os = "wasi"))]
    fn now_with(&self, _precision: Duration) -> Self::Instant {
        // On systems with no optimized form of `clock_gettime`, ignore the
        // precision argument.
        Self::Instant::from_std(time::Instant::now())
    }

    fn resolution(&self) -> Duration {
        let spec = clock_getres(ClockId::Realtime);
        Duration::new(
            spec.tv_sec.try_into().unwrap(),
            spec.tv_nsec.try_into().unwrap(),
        )
    }
}

#[cfg(windows)]
impl MonotonicClockExt for cap_primitives::time::MonotonicClock {
    type Instant = cap_primitives::time::Instant;

    fn now_with(&self, _precision: Duration) -> Self::Instant {
        // On systems with no optimized form of `clock_gettime`, ignore the
        // precision argument.
        Self::Instant::from_std(time::Instant::now())
    }

    fn resolution(&self) -> Duration {
        Duration::new(0, (*PERF_COUNTER_RES).try_into().unwrap())
    }
}

#[cfg(windows)]
lazy_static! {
    static ref PERF_COUNTER_RES: u64 = 1_000_000_000 / perf_counter_frequency().unwrap();
}
