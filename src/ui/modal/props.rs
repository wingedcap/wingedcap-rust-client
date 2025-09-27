use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use dioxus_tw_components::attributes::*;
use dioxus_tw_components_macro::UiComp;

use lucide_dioxus::X;

#[derive(Clone, Copy, Debug)]
pub struct ModalState {
    _open: bool,
    open: Option<bool>,
    onopenchange: Option<Callback<bool>>,
}

impl ModalState {
    fn new(open: Option<bool>, onopenchange: Option<Callback<bool>>) -> Self {
        Self {
            _open: false,
            open,
            onopenchange,
        }
    }

    fn is_open(&self) -> bool {
        match self.open {
            Some(open) => open,
            None => self._open,
        }
    }

    fn set_is_open(&mut self, open: bool) {
        match self.onopenchange {
            Some(onopenchange) => onopenchange(open),
            None => self._open = open,
        }
    }

    fn toggle(&mut self) {
        self.set_is_open(!self.is_open());
    }
}

impl IntoAttributeValue for ModalState {
    fn into_value(self) -> AttributeValue {
        match self.is_open() {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalProps {
    #[props(optional)]
    open: Option<bool>,

    #[props(optional)]
    onopenchange: Option<Callback<bool>>,

    children: Element,
}

impl std::default::Default for ModalProps {
    fn default() -> Self {
        Self {
            children: rsx! {},
            open: None,
            onopenchange: None,
        }
    }
}

/// Usage: \
/// ```ignore
/// Modal {
///     ModalTrigger {
///          "Open Modal"
///     }
///     ModalBackground {}
///     ModalContent {
///        div {
///             ModalClose { "X" }
///        }
///        div { class: "h4", "TITLE" }
///        div { class: "paragraph", "CONTENT" }
///    }
/// }
/// ```
#[component]
pub fn Modal(props: ModalProps) -> Element {
    let open = props.open;

    let mut state = use_signal(|| ModalState::new(open, props.onopenchange));

    use_effect(use_reactive!(|open| {
        state.write().open = open;
    }));

    use_context_provider(|| state);

    rsx! {
        {props.children}
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for ModalTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ModalTrigger(mut props: ModalTriggerProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
        props.onclick.call(event)
    };

    rsx! {
        div { onclick, ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalCloseProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    children: Element,
}

impl std::default::Default for ModalCloseProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: Ok(VNode::default()), // Default this way to be able to check the children in ModalClose
        }
    }
}

/// Div to close the content modal, by default it is a cross located at the top left corner of the modal
/// If you provide a children, it will be used instead of the default cross and no internal styling will be provided
#[component]
pub fn ModalClose(mut props: ModalCloseProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    let has_children = props.children != Ok(VNode::default());

    if !has_children {
        props.update_class_attribute();
    }

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        state.write().toggle();
    };

    rsx! {
        div { onclick, ..props.attributes,
            if !has_children {
                X { class: "size-4" }
            } else {
                {props.children}
            }
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,

    children: Element,
}

impl std::default::Default for ModalContentProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ModalContent(mut props: ModalContentProps) -> Element {
    let state = use_context::<Signal<ModalState>>();

    props.update_class_attribute();

    rsx! {
        div { "data-state": state.read().into_value(), ..props.attributes, {props.children} }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ModalBackgroundProps {
    #[props(optional, default = true)]
    interactive: bool,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub color: ReadOnlySignal<Color>,
    #[props(optional, default)]
    pub animation: ReadOnlySignal<Animation>,
    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for ModalBackgroundProps {
    fn default() -> Self {
        Self {
            interactive: true,
            attributes: Vec::<Attribute>::default(),
            color: ReadOnlySignal::<Color>::default(),
            animation: ReadOnlySignal::<Animation>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn ModalBackground(mut props: ModalBackgroundProps) -> Element {
    let mut state = use_context::<Signal<ModalState>>();

    props.update_class_attribute();

    let onclick = move |event: Event<MouseData>| {
        event.stop_propagation();
        if props.interactive {
            state.write().toggle();
            props.onclick.call(event)
        }
    };

    rsx! {
        div {
            "data-state": state.read().into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    }
}
