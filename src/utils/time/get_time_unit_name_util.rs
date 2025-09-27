use crate::types::TimeUnit;

pub fn get_time_unit_name(time_unit: TimeUnit) -> String {
    match time_unit {
        TimeUnit::Second => "seconds".to_string(),
        TimeUnit::Minute => "minutes".to_string(),
        TimeUnit::Hour => "hours".to_string(),
        TimeUnit::Day => "days".to_string(),
        TimeUnit::Week => "weeks".to_string(),
    }
}
