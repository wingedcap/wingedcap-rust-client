use dioxus::prelude::*;

use super::props::*;
use crate::types::*;

impl Class for TextAreaProps {
    fn base(&self) -> &'static str {
        "flex w-full px-2 py-1.5 text-sm text-foreground bg-background border rounded-md hover:brightness-105 focus:brightness-105 disabled:bg-muted disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:brightness-100"
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
}
