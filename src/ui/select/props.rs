use dioxus::{core::IntoAttributeValue, prelude::*};
use dioxus_core::AttributeValue;

use dioxus_tw_components_macro::UiComp;

use lucide_dioxus::{Check, ChevronDown};

use tailwind_fuse::tw_merge;

use chrono::{DateTime, Local};

use crate::types::*;

use super::super::popover::POPOVER_TARGET_ID;

#[derive(Clone)]
struct SelectState {
    value: String,
    on_value_change: Option<Callback<String>>,
    is_active: bool,
    last_hover: DateTime<Local>,
    is_hovered: bool,
}

impl SelectState {
    fn new(value: String, on_value_change: Option<Callback<String>>) -> Self {
        Self {
            value,
            on_value_change,
            is_active: false,
            last_hover: DateTime::default(),
            is_hovered: false,
        }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn get_is_active(&self) -> bool {
        self.is_active
    }

    fn set_last_hover(&mut self, last_hover: DateTime<Local>) {
        self.last_hover = last_hover;
    }

    fn set_is_hovered(&mut self, is_hovered: bool) {
        self.is_hovered = is_hovered;
    }

    fn set_value(&mut self, value: String) {
        match self.on_value_change.clone() {
            Some(on_value_change) => {
                on_value_change(value);
            }
            None => {
                self.value = value;
            }
        }
    }

    fn get_value(&self) -> String {
        self.value.clone()
    }
}

impl IntoAttributeValue for SelectState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props]
    value: String,

    #[props(optional)]
    on_value_change: Option<Callback<String>>,

    children: Element,
}

impl std::default::Default for SelectProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            value: String::new(),
            on_value_change: None,
            children: rsx! {},
        }
    }
}

#[component]
pub fn Select(mut props: SelectProps) -> Element {
    let props_clone = props.clone();

    props.update_class_attribute();

    let mut state =
        use_context_provider(|| Signal::new(SelectState::new(props.value, props.on_value_change)));

    let external_value = props_clone.value;

    use_effect(use_reactive!(|external_value| {
        state.write().value = external_value;
    }));

    let value = state.read().get_value();

    use_effect(move || {
        if let Some(on_value_change) = props.on_value_change {
            on_value_change(value.clone());
        }
    });

    rsx! {
        div { "data-state": state.read().clone().into_value(), ..props.attributes, {props.children} }

        if state.read().get_is_active() {
            div {
                class: "fixed top-0 left-0 w-full h-full bg-transparent",
                onclick: move |_event| {
                    state.write().close();
                },
            }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for SelectTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn SelectTrigger(mut props: SelectTriggerProps) -> Element {
    let mut state = use_context::<Signal<SelectState>>();

    props.update_class_attribute();

    let onclick = move |_: MouseEvent| {
        state.write().toggle();
        state.write().set_last_hover(Local::now());
        state.write().set_is_hovered(true);
    };

    rsx! {
        button {
            r#type: "button",
            role: "button",
            popovertarget: POPOVER_TARGET_ID,
            popovertargetaction: "show",
            "data-state": state.read().clone().into_value(),
            onclick,
            ..props.attributes,
            {props.children}
            ChevronDown { class: "size-4 text-muted-foreground ml-3" }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectValueProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

impl std::default::Default for SelectValueProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
        }
    }
}

#[component]
pub fn SelectValue(mut props: SelectValueProps) -> Element {
    props.update_class_attribute();

    let state = use_context::<Signal<SelectState>>();

    rsx! {
        span { ..props.attributes,"{state.read().value}" }
    }
}
#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadSignal<Animation>,

    children: Element,
}

impl std::default::Default for SelectContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            animation: ReadSignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn SelectContent(mut props: SelectContentProps) -> Element {
    let state = use_context::<Signal<SelectState>>();

    props.update_class_attribute();

    rsx! {
        div {
            id: POPOVER_TARGET_ID,
            popover: "manual",
            "data-state": state.read().clone().into_value(),
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SelectItemProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    value: String,

    children: Element,
}

#[component]
pub fn SelectItem(mut props: SelectItemProps) -> Element {
    props.update_class_attribute();

    let mut state = use_context::<Signal<SelectState>>();

    let value_clone = props.value.clone();

    let onclick = move |_: MouseEvent| {
        let value = props.value.clone();
        state.write().set_value(value);
        state.write().close();
    };

    rsx! {
        div { onclick, ..props.attributes,
            {props.children}
            Check {
                class: tw_merge!(
                    "size-4 ml-3", if state.read().get_value() == value_clone { "" } else {
                    "invisible" }
                ),
            }
        }
    }
}
