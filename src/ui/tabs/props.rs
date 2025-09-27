use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;
use dioxus_tw_components_macro::UiComp;

struct TabsState(String);

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TabsProps {
    #[props(optional)]
    default_tab: ReadOnlySignal<String>,

    #[props(optional)]
    on_change: Option<Callback<String>>,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TabsProps {
    fn default() -> Self {
        Self {
            default_tab: ReadOnlySignal::<String>::default(),
            on_change: None,
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn Tabs(mut props: TabsProps) -> Element {
    let state = use_signal(|| TabsState(props.default_tab.read().clone()));

    use_context_provider(|| state);

    let current_tab = state.read().0.clone();

    props.update_class_attribute();

    use_effect(use_reactive!(|current_tab| {
        let current_tab = current_tab.clone();

        if let Some(on_change) = props.on_change {
            on_change(current_tab);
        }
    }));

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TabsListProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for TabsListProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TabsList(mut props: TabsListProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TabsTriggerProps {
    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    children: Element,
}

impl std::default::Default for TabsTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            id: ReadOnlySignal::<String>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TabsTrigger(mut props: TabsTriggerProps) -> Element {
    let mut tab_state = use_context::<Signal<TabsState>>();

    props.update_class_attribute();

    let state = match tab_state.read().0 == *props.id.read() {
        true => "active",
        false => "inactive",
    };

    let onclick = move |_| {
        tab_state.set(TabsState(props.id.read().clone()));
    };

    rsx! {
        button { onclick, "data-state": state, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct TabsContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    id: ReadOnlySignal<String>,

    children: Element,
}

impl std::default::Default for TabsContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            id: ReadOnlySignal::<String>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn TabsContent(mut props: TabsContentProps) -> Element {
    let tab_state = use_context::<Signal<TabsState>>();

    props.update_class_attribute();

    let (state, is_hidden) = match tab_state.read().0 == *props.id.read() {
        true => ("active", false),
        false => ("inactive", true),
    };

    rsx! {
        if !is_hidden {
            div { "data-state": state, ..props.attributes, {props.children} }
        }
    }
}
