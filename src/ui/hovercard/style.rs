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
        "text-sm font-medium whitespace-nowrap transition-all cursor-default min-w-8 min-h-8 rounded-md flex items-center justify-center hover:bg-accent cursor-pointer"
    }
}

impl Class for HoverCardContentProps {
    fn base(&self) -> &'static str {
        "[inset:unset] block z-50 absolute bottom-full right-0 p-2 bg-gray-100 text-gray-900 -translate-y-0 font-normal border border-border max-w-40 text-xs w-max rounded-md shadow data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "transition-all duration-100 data-[state=inactive]:scale-90 data-[state=active]:scale-100 data-[state=inactive]:opacity-0"
            }
        })
    }
}
