use super::props::*;
use crate::types::*;

impl Class for TabsProps {}

impl Class for TabsListProps {
    fn base(&self) -> &'static str {
        "w-full flex h-10 p-1 items-center justify-center rounded-lg bg-muted/80 text-muted-foreground shadow-sm"
    }
}

impl Class for TabsTriggerProps {
    fn base(&self) -> &'static str {
        "flex grow items-center justify-center gap-1.5 whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium ring-offset-background transition-all duration-200
        data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm
        hover:text-foreground/80"
    }
}

impl Class for TabsContentProps {
    fn base(&self) -> &'static str {
        "mt-3 bg-transparent text-foreground"
    }
}
