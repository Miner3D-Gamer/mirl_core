/// Nanoseconds per microsecond
pub const NANOS_PER_MICRO: u64 = 1_000;
/// Nanoseconds per millisecond
pub const NANOS_PER_MILLI: u64 = 1_000_000;
/// Nanoseconds per second
pub const NANOS_PER_SEC: u64 = 1_000_000_000;

/// Microseconds per millisecond
pub const MICROS_PER_MILLI: u64 = 1_000;
/// Microseconds per second
pub const MICROS_PER_SEC: u64 = 1_000_000;

/// Milliseconds per second
pub const MILLIS_PER_SEC: u64 = 1_000;

/// Seconds per minute
pub const SECS_PER_MINUTE: u64 = 60;
/// Seconds per hour
pub const SECS_PER_HOUR: u64 = 3_600;
/// Seconds per day
pub const SECS_PER_DAY: u64 = 86_400;
/// Seconds per week
pub const SECS_PER_WEEK: u64 = 604_800;

/// Minutes per hour
pub const MINS_PER_HOUR: u64 = 60;
/// Minutes per day
pub const MINS_PER_DAY: u64 = 1_440;
/// Minutes per week
pub const MINS_PER_WEEK: u64 = 10_080;

/// Hours per day
pub const HOURS_PER_DAY: u64 = 24;
/// Hours per week
pub const HOURS_PER_WEEK: u64 = 168;

/// Days per week
pub const DAYS_PER_WEEK: u64 = 7;

/// Milliseconds per minute
pub const MILLIS_PER_MINUTE: u64 = 60_000;
/// Milliseconds per hour
pub const MILLIS_PER_HOUR: u64 = 3_600_000;
/// Milliseconds per day
pub const MILLIS_PER_DAY: u64 = 86_400_000;
/// Milliseconds per week
pub const MILLIS_PER_WEEK: u64 = 604_800_000;

/// Microseconds per minute
pub const MICROS_PER_MINUTE: u64 = 60_000_000;
/// Microseconds per hour
pub const MICROS_PER_HOUR: u64 = 3_600_000_000;
/// Microseconds per day
pub const MICROS_PER_DAY: u64 = 86_400_000_000;
/// Microseconds per week
pub const MICROS_PER_WEEK: u64 = 604_800_000_000;

/// Nanoseconds per minute
pub const NANOS_PER_MINUTE: u64 = 60_000_000_000;
/// Nanoseconds per hour
pub const NANOS_PER_HOUR: u64 = 3_600_000_000_000;
/// Nanoseconds per day
pub const NANOS_PER_DAY: u64 = 86_400_000_000_000;
/// Nanoseconds per week
pub const NANOS_PER_WEEK: u64 = 604_800_000_000_000;

/// Microseconds in one nanosecond
pub const MICRO_PER_NANO:f64 /* f16 is enough */ = 0.001;
/// Milliseconds in one nanosecond
pub const MILLI_PER_NANO:f64 /* f16 is enough */ = 0.000_001;
/// Seconds in one nanosecond
pub const SEC_PER_NANO: f64 /* f32 is enough */ = 0.000_000_001;

/// Milliseconds in one microsecond
pub const MILLI_PER_MICRO:f64 /* f16 is enough */ = 0.001;
/// Seconds in one microsecond
pub const SEC_PER_MICRO:f64 /* f16 is enough */ = 0.000_001;

/// Seconds in one millisecond
pub const SEC_PER_MILLI:f64 /* f16 is enough */ = 0.001;

/// Minutes in one second
pub const MINUTE_PER_SEC:f64 /* f16 is enough */ = 1.0 / 60.0;
/// Hours in one second
pub const HOUR_PER_SEC: f64 /* f32 is enough */ = 1.0 / 3_600.0;
/// Days in one second
pub const DAY_PER_SEC: f64 = 1.0 / 86_400.0;
/// Weeks in one second
pub const WEEK_PER_SEC: f64 = 1.0 / 604_800.0;

/// Hours in one minute
pub const HOUR_PER_MINUTE:f64 /* f16 is enough */ = 1.0 / 60.0;
/// Days in one minute
pub const DAY_PER_MINUTE: f64 /* f32 is enough */ = 1.0 / 1_440.0;
/// Weeks in one minute
pub const WEEK_PER_MINUTE: f64 = 1.0 / 10_080.0;

/// Days in one hour
pub const DAY_PER_HOUR:f64 /* f16 is enough */ = 1.0 / 24.0;
/// Weeks in one hour
pub const WEEK_PER_HOUR:f64 /* f16 is enough */ = 1.0 / 168.0;

/// Weeks in one day
pub const WEEK_PER_DAY:f64 /* f16 is enough */ = 1.0 / 7.0;
