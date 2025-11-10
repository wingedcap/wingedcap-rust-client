use dioxus::{dioxus_core::AttributeValue, prelude::Attribute};

use crate::types::UiComp;

pub trait GetClass {
    fn get_class(&mut self) -> String;
}

impl<T: UiComp> GetClass for T {
    fn get_class(&mut self) -> String {
        let class = match self.get_attributes() {
            Some(vec_attributes) => {
                match vec_attributes.iter_mut().find(|attr| attr.name == "class") {
                    Some(Attribute {
                        value: AttributeValue::Text(ref mut class_str),
                        ..
                    }) => Some(class_str.clone()),
                    _ => None,
                }
            }
            None => None,
        }
        .unwrap_or_default();

        class
    }
}
