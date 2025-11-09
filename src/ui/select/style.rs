use dioxus::prelude::*;

use super::props::*;
use crate::types::*;

impl Class for SelectProps {
    fn base(&self) -> &'static str {
        "z-20 text-foreground text-sm w-min min-w-28 h-9"
    }
}

impl Class for SelectTriggerProps {
    fn base(&self) -> &'static str {
        "bg-background rounded-lg whitespace-nowrap shadow-sm flex items-center text-left w-full h-full border border-border justify-between px-3 hover:border-border/80 transition-all focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-1"
    }
}

impl Class for SelectValueProps {
    fn base(&self) -> &'static str {
        ""
    }
}

impl Class for SelectContentProps {
    fn base(&self) -> &'static str {
        "z-50 p-1.5 cursor-default space-y-0.5 bg-background/95 min-w-28 flex flex-col rounded-lg border border-input [inset:unset] shadow-floating mt-1 whitespace-nowrap opacity-100 data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => {
                "transition-all duration-200 data-[state=inactive]:scale-95 data-[state=active]:scale-100 data-[state=inactive]:opacity-0 data-[state=inactive]:-translate-y-1"
            }
        })
    }
}

impl Class for SelectItemProps {
    fn base(&self) -> &'static str {
        "flex items-center text-left justify-between w-full px-2.5 hover:bg-accent/80 py-1.5 rounded-md transition-colors cursor-pointer"
    }
}
