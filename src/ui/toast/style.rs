use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for ToasterProps {
    fn base(&self) -> &'static str {
        "fixed z-50 w-full md:max-w-[420px] left-1/2 -translate-x-1/2 top-0 px-4 md:px-0"
    }
}

impl Class for Toast {
    fn base(&self) -> &'static str {
        "relative group bg-background/95 text-foreground text-sm border border-border/50 shadow-floating p-4 m-2 rounded-lg"
    }

    // Color is not a ReadOnlySignal so no need to read()
    fn color(&self) -> Option<&'static str> {
        Some(match self.color {
            Color::Primary => "bg-primary/95 text-primary-foreground border-primary/20",
            Color::Secondary => "bg-secondary/95 text-secondary-foreground border-secondary/20",
            Color::Destructive => {
                "bg-destructive/95 text-destructive-foreground border-destructive/20"
            }
            Color::Success => "bg-success/95 text-primary-foreground border-success/20",
            _ => "bg-background/95 text-foreground border-border/50",
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match self.animation {
            Animation::Light | Animation::Full => {
                "transition-all duration-300 ease-out data-[state=opening]:translate-y-full data-[state=opening]:opacity-0 data-[state=open]:translate-y-0 data-[state=open]:opacity-100 data-[state=closing]:-translate-y-full data-[state=closing]:opacity-0"
            }
            _ => "",
        })
    }
}
