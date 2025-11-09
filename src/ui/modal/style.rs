use dioxus::prelude::*;

use super::props::*;
use crate::types::*;

impl Class for ModalProps {}

// Used to make a "useless" div which does not create a newline that wrap our trigger with our trigger_closure
// Also used by ModalCancelProps
impl Class for ModalTriggerProps {
    fn base(&self) -> &'static str {
        "inline-flex items-center cursor-pointer justify-center size-8 whitespace-nowrap rounded-lg text-sm transition-all hover:bg-accent/80 hover:text-accent-foreground active:scale-95"
    }
}

impl Class for ModalCloseProps {
    fn base(&self) -> &'static str {
        "absolute top-4 right-4 rounded-lg border border-transparent cursor-pointer hover:bg-accent active:border-border transition-all active:scale-95"
    }
}

impl Class for ModalContentProps {
    fn base(&self) -> &'static str {
        "p-6 flex flex-col top-[50%] left-[50%] z-50 min-w-80 max-w-[calc(100vw-2rem)] max-h-[calc(100vh-2rem)] bg-background border border-border/50 rounded-xl shadow-dialog fixed translate-x-[-50%] translate-y-[-50%] data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "data-[state=inactive]:translate-y-4 data-[state=inactive]:opacity-0 data-[state=inactive]:scale-95 transition-all duration-300 ease-out"
            }
        })
    }
}

impl Class for ModalBackgroundProps {
    fn base(&self) -> &'static str {
        "w-full h-full top-0 left-0 z-40 opacity-60 fixed data-[state=inactive]:invisible"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "bg-primary",
            Color::Secondary => "bg-secondary",
            Color::Destructive => "bg-destructive",
            Color::Success => "bg-success",
            _ => "bg-foreground/80",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => {
                "data-[state=inactive]:opacity-0 data-[state=inactive]:invisible transition-all duration-300"
            }
        })
    }
}
