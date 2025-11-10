use std::str::FromStr;

use dioxus::prelude::*;

use super::props::*;
use crate::types::*;

impl Class for ButtonProps {
    fn base(&self) -> &'static str {
        "inline-flex items-center cursor-pointer justify-center whitespace-nowrap rounded-lg text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed"
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
            Size::Lg => "h-11 rounded-lg px-8",
            Size::Xl => "h-14 px-9 py-3 text-xl",
        })
    }

    fn variant(&self) -> Option<&'static str> {
        Some(match *self.variant.read() {
            ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90 shadow-sm hover:shadow-md active:scale-95",
            ButtonVariant::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground hover:border-accent-foreground/20 active:scale-95"
            }
            ButtonVariant::Ghost => "hover:bg-accent/80 hover:text-accent-foreground active:scale-95",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline hover:text-primary/80",
            ButtonVariant::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90 shadow-sm hover:shadow-md active:scale-95"
            }
        })
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "",
            Animation::Light | Animation::Full => "transition-all duration-200",
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
