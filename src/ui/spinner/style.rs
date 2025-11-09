use super::props::*;
use crate::types::*;

impl Class for SpinnerProps {
    fn base(&self) -> &'static str {
        "animate-spin size-4"
    }
}
