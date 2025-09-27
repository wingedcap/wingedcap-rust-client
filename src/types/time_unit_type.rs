use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}
