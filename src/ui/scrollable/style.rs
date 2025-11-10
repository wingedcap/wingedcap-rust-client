use super::props::*;
use dioxus::prelude::*;
// use dioxus_tw_components::attributes::*;

impl Class for ScrollableProps {
    fn base(&self) -> &'static str {
        "scrollbar p-2 border border-border rounded-md min-w-32"
    }

    fn orientation(&self) -> Option<&'static str> {
        Some(match *self.orientation.read() {
            Orientation::Horizontal => {
                "overflow-y-auto overflow-x-hidden -rotate-90 origin-[right_top] -rotate-90"
            }
            Orientation::Vertical => "overflow-y-auto overflow-x-hidden grid-flow-row",
        })
    }
}
