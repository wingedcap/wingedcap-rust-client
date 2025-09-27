use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for CardProps {
    fn base(&self) -> &'static str {
        "bg-card text-card-foreground shadow-floating rounded-md border-none p-6"
    }
}

impl Class for CardHeaderProps {
    fn base(&self) -> &'static str {
        "flex flex-col space-y-1.5"
    }
}

impl Class for CardTitleProps {
    fn base(&self) -> &'static str {
        "text-xl font-semibold leading-none tracking-tight"
    }
}

impl Class for CardDescriptionProps {
    fn base(&self) -> &'static str {
        "text-muted-foreground text-sm"
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
