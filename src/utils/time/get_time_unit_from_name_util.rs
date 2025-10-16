use crate::types::TimeUnit;

pub fn get_time_unit_from_name(time_unit: String) -> Result<TimeUnit, String> {
    match time_unit.as_str() {
        "seconds" => Ok(TimeUnit::Second),
        "minutes" => Ok(TimeUnit::Minute),
        "hours" => Ok(TimeUnit::Hour),
        "days" => Ok(TimeUnit::Day),
        "weeks" => Ok(TimeUnit::Week),
        _ => Err(format!("Invalid time unit: {}", time_unit)),
    }
}
