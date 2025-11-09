use dioxus::{core::IntoAttributeValue, prelude::*};
use dioxus_core::AttributeValue;

use dioxus_tw_components_macro::UiComp;

use chrono::{DateTime, Local, TimeDelta};

use crate::utils::time::wait;

use crate::types::*;

use super::super::popover::POPOVER_TARGET_ID;

#[derive(Clone, Debug)]
pub struct HoverState {
    is_active: bool,
    last_hover: DateTime<Local>,
    closing_delay_ms: TimeDelta,
}

impl HoverState {
    fn new(closing_delay_ms: u32) -> Self {
        Self {
            is_active: false,
            closing_delay_ms: TimeDelta::milliseconds(closing_delay_ms as i64),
            last_hover: DateTime::default(),
        }
    }

    fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    fn close(&mut self) {
        self.is_active = false;
    }

    fn set_is_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    fn get_is_active(&self) -> bool {
        self.is_active
    }

    fn set_last_hover(&mut self, last_hover: DateTime<Local>) {
        self.last_hover = last_hover;
    }

    fn get_last_hover(&self) -> DateTime<Local> {
        self.last_hover
    }

    fn get_closing_delay(&self) -> TimeDelta {
        self.closing_delay_ms
    }
}

impl IntoAttributeValue for HoverState {
    fn into_value(self) -> AttributeValue {
        match self.is_active {
            true => AttributeValue::Text("active".to_string()),
            false => AttributeValue::Text("inactive".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct HoverCardProps {
    /// Corresponds to the time in ms it takes for the toggle to close itself if not active
    #[props(default = 100)]
    closing_delay_ms: u32,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for HoverCardProps {
    fn default() -> Self {
        Self {
            closing_delay_ms: 500,
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn HoverCard(mut props: HoverCardProps) -> Element {
    let mut state = use_context_provider(|| Signal::new(HoverState::new(props.closing_delay_ms)));

    props.update_class_attribute();

    let onmouseleave = move |_| {
        state.write().set_last_hover(Local::now());
        state.write().set_is_active(false);

        let closing_delay_ms = state.read().closing_delay_ms;

        let duration_in_ms = closing_delay_ms
            .num_milliseconds()
            .try_into()
            .unwrap_or_default();

        spawn(async move {
            wait(duration_in_ms).await;

            let is_active = state.read().get_is_active();

            let last_hover = state.read().get_last_hover();
            let now = Local::now();
            let dt = state.read().get_closing_delay();

            if !is_active && now - last_hover >= dt {
                state.write().close();
            }
        });
    };

    rsx! {
        div {
            "data-state": state.into_value(),
            onmouseleave,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct HoverCardTriggerProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

impl std::default::Default for HoverCardTriggerProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            onclick: EventHandler::<MouseEvent>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn HoverCardTrigger(mut props: HoverCardTriggerProps) -> Element {
    let mut state = use_context::<Signal<HoverState>>();

    props.update_class_attribute();

    // We need this event here to not close the hover card when clicking its content
    let onclick = move |event| {
        state.write().toggle();
        props.onclick.call(event);
    };

    rsx! {
        button {
            r#type: "button",
            popovertarget: POPOVER_TARGET_ID,
            popovertargetaction: "show",
            role: "button",
            "data-state": state.into_value(),
            onclick,
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct HoverCardContentProps {
    #[props(optional, default)]
    pub side: Side,

    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, default)]
    pub animation: ReadSignal<Animation>,

    children: Element,
}

impl std::default::Default for HoverCardContentProps {
    fn default() -> Self {
        Self {
            side: Side::Right,
            attributes: Vec::<Attribute>::default(),
            animation: ReadSignal::<Animation>::default(),
            children: rsx! {},
        }
    }
}

#[component]
pub fn HoverCardContent(mut props: HoverCardContentProps) -> Element {
    let state = use_context::<Signal<HoverState>>();

    props.update_class_attribute();

    rsx! {
        div {
            id: POPOVER_TARGET_ID,
            popover: "manual",
            "data-state": state.into_value(),
            ..props.attributes,
            {props.children}
        }
    }
}
