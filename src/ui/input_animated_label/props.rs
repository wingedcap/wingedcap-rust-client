use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;
use dioxus_tw_components_macro::UiComp;
use tailwind_fuse::tw_merge;

use crate::{types::GetClass, ui::input::Input};

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct InputAnimatedLabelProps {
    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    // Core props for the animated label input
    #[props]
    pub label: String,

    #[props(optional)]
    pub placeholder: Option<String>,

    #[props(optional)]
    pub value: Option<String>,

    // Custom class names for different parts
    #[props(optional)]
    pub container_class: Option<String>,

    #[props(optional)]
    pub label_class: Option<String>,

    #[props(optional)]
    pub label_background_class: Option<String>,

    // Event handlers
    #[props(optional)]
    pub oninput: Option<EventHandler<FormEvent>>,

    #[props(optional)]
    pub onblur: Option<EventHandler<FocusEvent>>,

    #[props(optional)]
    pub onkeypress: Option<EventHandler<KeyboardEvent>>,

    #[props(optional)]
    pub onmounted: Option<EventHandler<Event<MountedData>>>,
}

#[component]
pub fn InputAnimatedLabel(mut props: InputAnimatedLabelProps) -> Element {
    let label_text = props.label.clone();
    let placeholder_text = props.placeholder.clone().unwrap_or(label_text.clone());

    // Default classes for the floating label
    let default_label_class ="origin-start text-gray-500 group-focus-within:text-blue-600 has-[+input:not(:placeholder-shown)]:text-gray-700 absolute top-1/2 block -translate-y-1/2 cursor-text px-1 text-sm transition-all duration-200 group-focus-within:pointer-events-none group-focus-within:top-0 group-focus-within:cursor-default group-focus-within:text-xs group-focus-within:font-medium has-[+input:not(:placeholder-shown)]:pointer-events-none has-[+input:not(:placeholder-shown)]:top-0 has-[+input:not(:placeholder-shown)]:cursor-default has-[+input:not(:placeholder-shown)]:text-xs has-[+input:not(:placeholder-shown)]:font-medium";

    // Combine default with custom styles using tw_merge
    let final_label_class = tw_merge!(
        default_label_class,
        props.label_class.as_deref().unwrap_or("")
    );

    let final_container_class = tw_merge!(
        "group relative",
        props.container_class.as_deref().unwrap_or("")
    );

    let final_label_background_class = tw_merge!(
        "bg-background inline-flex px-2",
        props.label_background_class.as_deref().unwrap_or("")
    );

    let class = props.get_class();

    let final_input_class = tw_merge!("placeholder:text-transparent", class);

    let label_text_for_label = label_text.clone();
    let label_text_for_span = label_text.clone();
    let label_text_for_input = label_text.clone();

    let handle_change = move |e: FormEvent| {
        let oninput = props.oninput.clone();

        if let Some(oninput) = oninput {
            oninput.call(e);
        }
    };

    rsx! {
        div { class: final_container_class,
            label { r#for: label_text_for_label, class: final_label_class,
                span { class: final_label_background_class, {label_text_for_span} }
            }

            Input {
                id: label_text_for_input,
                placeholder: placeholder_text,
                class: final_input_class,
                value: props.value.clone().unwrap_or_default(),
                oninput: handle_change,
            }
        }
    }
}
