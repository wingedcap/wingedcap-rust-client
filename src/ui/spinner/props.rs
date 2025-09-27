use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;
use dioxus_tw_components_macro::UiComp;

use lucide_dioxus::Loader;

use crate::types::GetClass;

#[derive(Default, Clone, PartialEq, Props, UiComp)]
pub struct SpinnerProps {
    #[props(extends = span, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Spinner(mut props: SpinnerProps) -> Element {
    props.update_class_attribute();

    let class = props.get_class();

    rsx! {
        Loader { class }
    }
}
