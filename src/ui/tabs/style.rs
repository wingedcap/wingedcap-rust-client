use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for TabsProps {}

impl Class for TabsListProps {
    fn base(&self) -> &'static str {
        "w-full flex h-8 p-1 items-center justify-center rounded-md bg-muted text-muted-foreground"
    }
}

impl Class for TabsTriggerProps {
    fn base(&self) -> &'static str {
        "flex grow items-center justify-center whitespace-nowrap rounded-md px-2 py-0.5 text-sm font-semibold ring-offset-background transition-all duration-75
        data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow"
    }
}

impl Class for TabsContentProps {
    fn base(&self) -> &'static str {
        "mt-2 bg-background text-foreground border border-border rounded-md"
    }
}
