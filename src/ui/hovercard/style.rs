use super::props::*;
use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;

impl Class for HoverCardProps {
    fn base(&self) -> &'static str {
        "group relative text-foreground"
    }
}

impl Class for HoverCardTriggerProps {
    fn base(&self) -> &'static str {
        "text-sm font-medium whitespace-nowrap transition-all cursor-default min-w-8 min-h-8 rounded-lg flex items-center justify-center hover:bg-accent/70 cursor-pointer active:scale-95"
    }
}

impl Class for HoverCardContentProps {
    fn base(&self) -> &'static str {
        "[inset:unset] block z-50 absolute bottom-full p-3 bg-foreground/95 text-background -translate-y-1 font-normal border border-border/20 max-w-40 text-xs w-max rounded-lg shadow-floating data-[state=inactive]:invisible"
    }

    fn side(&self) -> Option<&'static str> {
        match self.side.clone() {
            Side::Left => Some("left-0"),
            Side::Top => Some("left-1/2 -translate-x-1/2"),
            _ => Some("right-0"),
        }
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "transition-all duration-200 data-[state=inactive]:scale-95 data-[state=active]:scale-100 data-[state=inactive]:opacity-0 data-[state=inactive]:translate-y-1"
            }
        })
    }
}
