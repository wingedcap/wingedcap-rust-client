use super::props::*;
use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;

impl Class for InputProps {
    fn base(&self) -> &'static str {
        "border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
    }

    fn color(&self) -> Option<&'static str> {
        Some(match *self.color.read() {
            Color::Primary => "border-primary",
            Color::Secondary => "border-secondary",
            Color::Destructive => "border-destructive",
            Color::Success => "border-success",
            _ => "border-input",
        })
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "h-4 text-xs",
            Size::Sm => "h-6 text-xs",
            Size::Md => "h-8 text-sm",
            Size::Lg => "h-11 text-base",
            Size::Xl => "h-14 text-lg",
        })
    }
}
