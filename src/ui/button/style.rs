use std::str::FromStr;

use super::props::*;
use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;

impl Class for ButtonProps {
    fn base(&self) -> &'static str {
        "inline-flex items-center cursor-pointer justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50"
    }

    // Handled in variant
    fn color(&self) -> Option<&'static str> {
        Some("")
    }

    fn size(&self) -> Option<&'static str> {
        Some(match *self.size.read() {
            Size::Xs => "size-6",
            Size::Sm => "size-7 rounded-md px-3",
            Size::Md => "size-8 px-4 py-2",
            Size::Lg => "h-11 rounded-md px-8",
            Size::Xl => "h-14 px-9 py-3 text-xl",
        })
    }

    fn variant(&self) -> Option<&'static str> {
        Some(match *self.variant.read() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => "transition-colors duration-150",
            // Animation::Full => "relative z-30 after:-z-20 after:absolute after:h-1 after:w-1 after:bg-background/80 after:left-5 overflow-hidden after:bottom-0 after:translate-y-full after:rounded-md hover:after:scale-300 hover:after:transition-all hover:after:duration-700 after:transition-all after:duration-700 transition-all duration-700",
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Ghost,
    Link,
    Destructive,
}

impl FromStr for ButtonVariant {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "outline" => Ok(ButtonVariant::Outline),
            "ghost" => Ok(ButtonVariant::Ghost),
            "link" => Ok(ButtonVariant::Link),
            _ => Ok(ButtonVariant::Default),
        }
    }
}

impl std::fmt::Display for ButtonVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ButtonVariant::Default => "Default",
            ButtonVariant::Outline => "Outline",
            ButtonVariant::Ghost => "Ghost",
            ButtonVariant::Link => "Link",
            ButtonVariant::Destructive => "Destructive",
        };
        f.write_str(s)
    }
}
