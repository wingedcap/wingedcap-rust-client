use super::TimeUnit;

#[derive(Clone, Debug)]
pub struct Time {
    pub magnitude: u64,
    pub unit: TimeUnit,
}
