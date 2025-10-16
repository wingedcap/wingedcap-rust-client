use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for CardProps {
    fn base(&self) -> &'static str {
        "bg-card/95 text-card-foreground shadow-card hover:shadow-floating transition-shadow duration-300 rounded-lg border border-border/50 p-6"
    }
}

impl Class for CardHeaderProps {
    fn base(&self) -> &'static str {
        "flex flex-col space-y-2"
    }
}

impl Class for CardTitleProps {
    fn base(&self) -> &'static str {
        "text-2xl font-semibold leading-tight tracking-tight text-foreground"
    }
}

impl Class for CardDescriptionProps {
    fn base(&self) -> &'static str {
        "text-muted-foreground text-sm leading-relaxed"
    }
}

impl Class for CardContentProps {
    fn base(&self) -> &'static str {
        "pt-0"
    }
}

impl Class for CardFooterProps {
    fn base(&self) -> &'static str {
        "flex items-center pt-10"
    }
}
