use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for ToasterProps {
    fn base(&self) -> &'static str {
        "fixed z-50 w-full md:max-w-[400px] left-1/2 -translate-x-1/2 top-0"
    }
}

impl Class for Toast {
    fn base(&self) -> &'static str {
        "relative group bg-background text-foreground text-sm border border-border shadow p-4 m-2 rounded-md"
    }

    // Color is not a ReadOnlySignal so no need to read()
    fn color(&self) -> Option<&'static str> {
        Some(match self.color {
            Color::Primary => "bg-primary text-primary",
            Color::Secondary => "bg-secondary text-secondary",
            Color::Destructive => "bg-destructive text-destructive",
            Color::Success => "bg-success text-primary",
            _ => "bg-background text-foreground",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::Light | Animation::Full => {
                "transition-all duration-150 data-[state=opening]:translate-y-full data-[state=open]:translate-y-0 data-[state=closing]:-translate-y-[200%] bg-background"
            }
            _ => "",
        })
    }
}
