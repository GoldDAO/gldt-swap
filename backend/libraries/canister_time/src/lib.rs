use ic_cdk_timers::TimerId;
use std::time::Duration;

use time::{OffsetDateTime, Time, Weekday};
use types::{Milliseconds, Second, TimestampMillis, TimestampNanos};

pub const SECOND_IN_MS: Milliseconds = 1000;
pub const MINUTE_IN_MS: Milliseconds = SECOND_IN_MS * 60;
pub const HOUR_IN_MS: Milliseconds = MINUTE_IN_MS * 60;
pub const DAY_IN_MS: Milliseconds = HOUR_IN_MS * 24;
pub const WEEK_IN_MS: Milliseconds = DAY_IN_MS * 7;

pub const DAY_IN_SECONDS: Second = 24 * 60 * 60;

pub const NANOS_PER_MILLISECOND: u64 = 1_000_000;

pub fn timestamp_seconds() -> u64 {
    timestamp_nanos() / 1_000_000_000
}

pub fn timestamp_millis() -> u64 {
    timestamp_nanos() / 1_000_000
}

pub fn timestamp_micros() -> u64 {
    timestamp_nanos() / 1_000
}

#[cfg(target_arch = "wasm32")]
pub fn timestamp_nanos() -> u64 {
    unsafe { ic0::time() as u64 }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn timestamp_nanos() -> u64 {
    use std::time::SystemTime;

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

pub fn now_millis() -> TimestampMillis {
    now_nanos() / NANOS_PER_MILLISECOND
}

#[cfg(target_arch = "wasm32")]
pub fn now_nanos() -> TimestampNanos {
    ic_cdk::api::time()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn now_nanos() -> TimestampNanos {
    0
}

pub fn run_now_then_interval(interval: Duration, func: fn()) -> TimerId {
    ic_cdk_timers::set_timer(Duration::ZERO, func);
    ic_cdk_timers::set_timer_interval(interval, func)
}

pub fn run_interval(interval: Duration, func: fn()) {
    ic_cdk_timers::set_timer_interval(interval, func);
}

pub fn run_once(func: fn()) {
    ic_cdk_timers::set_timer(Duration::ZERO, func);
}

fn calculate_next_timestamp(hour: u8) -> Option<u64> {
    if hour > 23 {
        return None;
    }

    let now = OffsetDateTime::from_unix_timestamp((now_millis() / 1000) as i64).ok()?;
    let target_time = Time::from_hms(hour, 0, 0).ok()?;

    let next_occurrence = if now.time().hour() >= hour {
        // Target hour has passed today, get tomorrow's date at target hour
        now.saturating_add(time::Duration::days(1))
            .replace_time(target_time)
    } else {
        // Target hour hasn't passed, get today's date at target hour
        now.replace_time(target_time)
    };

    Some(next_occurrence.unix_timestamp() as u64 * 1000) // Convert to milliseconds
}

pub fn start_job_daily_at(hour: u8, func: fn()) {
    if let Some(next_timestamp) = calculate_next_timestamp(hour) {
        let now_millis = now_millis();

        if next_timestamp > now_millis {
            let delay = Duration::from_millis(next_timestamp - now_millis);

            let timer_func = move || {
                run_now_then_interval(Duration::from_millis(DAY_IN_MS), func);
            };

            ic_cdk_timers::set_timer(delay, timer_func);

            tracing::info!(
                "Job scheduled to start at the next {}:00. (Timestamp: {})",
                hour,
                next_timestamp
            );
        } else {
            tracing::error!("Failed to calculate a valid timestamp for the next job.");
        }
    } else {
        tracing::error!("Invalid hour provided for job scheduling: {}", hour);
    }
}

fn calculate_next_weekday_timestamp(
    weekday: Weekday,
    hour: u8,
    now_fn: impl Fn() -> u64,
) -> Option<u64> {
    if hour > 23 {
        return None;
    }

    let now_millis = now_fn();
    let now = OffsetDateTime::from_unix_timestamp((now_millis / 1000) as i64).ok()?;
    let target_time = Time::from_hms(hour, 0, 0).ok()?;

    let mut next_occurrence = now.replace_time(target_time);
    while next_occurrence.weekday() != weekday || next_occurrence < now {
        next_occurrence = next_occurrence.saturating_add(time::Duration::days(1));
    }

    Some(next_occurrence.unix_timestamp() as u64 * 1000)
}

pub fn start_job_weekly_at(weekday: Weekday, hour: u8, func: fn(), now_fn: &impl Fn() -> u64) {
    if let Some(next_timestamp) = calculate_next_weekday_timestamp(weekday, hour, now_fn) {
        let now_millis = now_fn();

        if next_timestamp > now_millis {
            let delay = Duration::from_millis(next_timestamp - now_millis);

            let timer_func = move || {
                run_now_then_interval(Duration::from_millis(DAY_IN_MS * 7), func);
            };

            ic_cdk_timers::set_timer(delay, timer_func);

            tracing::info!(
                "Job scheduled to start on {:?} at {}:00. (Timestamp: {})",
                weekday,
                hour,
                next_timestamp
            );
        } else {
            tracing::error!("Failed to calculate a valid timestamp for the next weekly job.");
        }
    } else {
        tracing::error!("Invalid hour provided for weekly job scheduling: {}", hour);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_calculate_next_timestamp() {
        // Mock current time: Sat Nov 23 2024 10:52:11 UTC
        // 1732355531
        // 1732362731
        //       7128
        let now = datetime!(2024-11-23 10:52:11 UTC);

        let target_hour = 12; // Next target hour is 12 o'clock

        // Expected delay: 15 hours from 20:52:11 -> 12:00:00 next day
        let expected_delay = 12 * 3600 * 1000;

        let calculated_delay =
            calculate_next_timestamp(target_hour).expect("Failed to calculate next timestamp");

        println!("Calculated delay: {} milliseconds", calculated_delay);

        // Verify the calculated delay matches the expected delay
        assert_eq!(
            calculated_delay, expected_delay,
            "Expected delay: {}, Calculated delay: {}",
            expected_delay, calculated_delay
        );
    }

    #[test]
    fn test_calculate_next_weekday_timestamp() {
        use time::{OffsetDateTime, Weekday};

        // test 1
        let mock_now = || {
            let fixed_time = OffsetDateTime::parse(
                "2025-02-28T16:00:00Z", // Friday 1 hour
                &time::format_description::well_known::Rfc3339,
            )
            .unwrap();
            fixed_time.unix_timestamp() as u64 * 1000
        };

        let mock_func = || {
            tracing::info!("Weekly job executed!");
        };

        // Start job for next Friday at 3:00 PM
        let res = calculate_next_weekday_timestamp(Weekday::Friday, 15, &mock_now).unwrap();

        assert_eq!(res, 1741359600000); // 7 Mar 2025, 15:00:00

        // test 2
        let mock_now = || {
            let fixed_time = OffsetDateTime::parse(
                "2025-02-27T14:55:00Z", // Friday 1 hour
                &time::format_description::well_known::Rfc3339,
            )
            .unwrap();
            fixed_time.unix_timestamp() as u64 * 1000
        };

        let mock_func = || {
            tracing::info!("Weekly job executed!");
        };

        // Start job for next Friday at 3:00 PM
        let res = calculate_next_weekday_timestamp(Weekday::Friday, 15, &mock_now).unwrap();

        assert_eq!(res, 1740754800000); // 28 Feb 2025, 15:00:00
    }
}
