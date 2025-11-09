use dioxus::prelude::*;

use super::props::*;
use crate::types::*;

impl Class for SeparatorProps {
    fn base(&self) -> &'static str {
        "bg-border/60 shrink-0"
    }

    fn orientation(&self) -> Option<&'static str> {
        Some(match *self.orientation.read() {
            Orientation::Horizontal => "w-full h-px",
            Orientation::Vertical => "h-full w-px",
        })
    }
}
