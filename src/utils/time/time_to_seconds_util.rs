use crate::types::{Time, TimeUnit};

const SECONDS_IN_MINUTE: u64 = 60;
const MINUTES_IN_HOUR: u64 = 60;
const HOURS_IN_DAY: u64 = 24;
const DAYS_IN_WEEK: u64 = 7;

const SECONDS_IN_HOUR: u64 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;
const SECONDS_IN_DAY: u64 = SECONDS_IN_HOUR * HOURS_IN_DAY;
const SECONDS_IN_WEEK: u64 = SECONDS_IN_DAY * DAYS_IN_WEEK;

pub fn time_to_seconds(time: Time) -> u64 {
    time.magnitude
        * match time.unit {
            TimeUnit::Second => 1,
            TimeUnit::Minute => SECONDS_IN_MINUTE,
            TimeUnit::Hour => SECONDS_IN_HOUR,
            TimeUnit::Day => SECONDS_IN_DAY,
            TimeUnit::Week => SECONDS_IN_WEEK,
        }
}
