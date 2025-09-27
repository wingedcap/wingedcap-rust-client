use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for SpinnerProps {
    fn base(&self) -> &'static str {
        "animate-spin size-4"
    }
}
