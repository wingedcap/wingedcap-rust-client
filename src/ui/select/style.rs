use super::props::*;
use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;

impl Class for SelectProps {
    fn base(&self) -> &'static str {
        "z-20 text-foreground text-sm w-min min-w-28 h-8"
    }
}

impl Class for SelectTriggerProps {
    fn base(&self) -> &'static str {
        "bg-background rounded-md whitespace-nowrap shadow-sm flex items-center text-left w-full h-full border border-border justify-between px-3"
    }
}

impl Class for SelectValueProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl Class for SelectContentProps {
    fn base(&self) -> &'static str {
        "z-50 p-1 cursor-default space-y-0 bg-background min-w-28 flex flex-col rounded-md border border-input [inset:unset] shadow mt-0.5 whitespace-nowrap opacity-100 data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => {
                "transition-all duration-100 data-[state=inactive]:scale-90 data-[state=active]:scale-100 data-[state=inactive]:opacity-0"
            }
        })
    }
}

impl Class for SelectItemProps {
    fn base(&self) -> &'static str {
        "flex items-center text-left justify-between w-full px-2 hover:bg-accent py-1 rounded-sm"
    }
}
