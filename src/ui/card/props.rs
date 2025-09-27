use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;
use dioxus_tw_components_macro::UiComp;

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CardProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

// impl std::default::Default for CardProps {
//     fn default() -> Self {
//         Self {
//             attributes: Vec::<Attribute>::default(),
//             children: rsx! {},
//         }
//     }
// }

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CardHeaderProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CardTitleProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CardDescriptionProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CardContentProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct CardFooterProps {
    #[props(extends = div, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Card(mut props: CardProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[component]
pub fn CardHeader(mut props: CardHeaderProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[component]
pub fn CardTitle(mut props: CardTitleProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[component]
pub fn CardDescription(mut props: CardDescriptionProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

#[component]
pub fn CardContent(mut props: CardContentProps) -> Element {
    props.update_class_attribute();

    rsx! {
        div { ..props.attributes,{props.children} }
    }
}

// #[component]
// pub fn CardFooter(mut props: CardFooterProps) -> Element {
//     props.update_class_attribute();

//     rsx! {
//         div { ..props.attributes,{props.children} }
//     }
// }
