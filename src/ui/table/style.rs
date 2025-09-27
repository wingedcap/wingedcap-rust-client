use super::props::*;
use dioxus_tw_components::attributes::*;

impl Class for TableProps {
    fn base(&self) -> &'static str {
        "w-full caption-bottom text-sm"
    }
}

impl Class for TableHeaderProps {
    fn base(&self) -> &'static str {
        "[&_tr]:border-b"
    }
}

impl Class for TableBodyProps {
    fn base(&self) -> &'static str {
        "[&_tr:last-child]:border-0"
    }
}

impl Class for TableFooterProps {
    fn base(&self) -> &'static str {
        "bg-muted/50 border-t font-medium [&>tr]:last:border-b-0"
    }
}

impl Class for TableHeadProps {
    fn base(&self) -> &'static str {
        "text-muted-foreground h-12 px-4 text-left align-middle font-medium [&:has([role=checkbox])]:pr-0"
    }
}

impl Class for TableRowProps {
    fn base(&self) -> &'static str {
        "hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors"
    }
}

impl Class for TableCellProps {
    fn base(&self) -> &'static str {
        "p-4 align-middle [&:has([role=checkbox])]:pr-0"
    }
}

impl Class for TableCaptionProps {
    fn base(&self) -> &'static str {
        "text-muted-foreground mt-4 text-sm"
    }
}
