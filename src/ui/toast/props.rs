use dioxus::prelude::*;

use dioxus_tw_components_macro::UiComp;

use crate::utils::time::wait;

use lucide_dioxus::X;

use crate::types::*;

use crate::utils::use_unique_id;

/// Used to keep track of all the current toasts, for now it only keeps 1 Toast
#[derive(Default)]
pub struct ToasterState {
    pub toasts: Vec<Toast>,
    // pub shape: Toast,
}

pub trait ToastRenderer {
    // fn description(&mut self, description: Element) -> &mut Self;
    // fn color(&mut self, color: Color) -> &mut Self;
    // fn title(&mut self, title: impl ToString) -> &mut Self;
    // fn duration_in_ms(&mut self, duration: u32) -> &mut Self;
    // fn animation(&mut self, animation: Animation) -> &mut Self;
    // fn is_closable(&mut self, is_closable: bool) -> &mut Self;
    // fn success(&mut self, description: impl ToString);
    fn error(&mut self, description: impl ToString);
    // fn loading(&mut self, description: impl ToString);
}

impl ToastRenderer for Signal<ToasterState> {
    // fn description(&mut self, description: Element) -> &mut Self {
    //     let shape = self.peek().shape.clone();
    //     self.write().shape = shape.description(description);
    //     self
    // }

    // fn color(&mut self, color: Color) -> &mut Self {
    //     let shape = self.peek().shape.clone();
    //     self.write().shape = shape.color(color);
    //     self
    // }

    // fn title(&mut self, title: impl ToString) -> &mut Self {
    //     let shape = self.peek().shape.clone();
    //     self.write().shape = shape.title(title);
    //     self
    // }

    // fn duration_in_ms(&mut self, duration: u32) -> &mut Self {
    //     let shape = self.peek().shape.clone();
    //     self.write().shape = shape.duration_in_ms(duration);
    //     self
    // }

    // fn animation(&mut self, animation: Animation) -> &mut Self {
    //     let shape = self.peek().shape.clone();
    //     self.write().shape = shape.animation(animation);
    //     self
    // }

    // fn is_closable(&mut self, is_closable: bool) -> &mut Self {
    //     let shape = self.peek().shape.clone();
    //     self.write().shape = shape.is_closable(is_closable);
    //     self
    // }

    // /// Build a toast with success background color and title "Success"
    // /// The string passed as argument will be the description of the Toast
    // fn success(&mut self, description: impl ToString) {
    //     let toast = Toast::default()
    //         .title(String::from("Success"))
    //         .color(Color::Success)
    //         .description(rsx! {
    //             p { "{description.to_string()}" }
    //         });
    //     self.write().toasts.push(toast);
    // }

    /// Build a toast with destructive background color and title "Error"
    /// The string passed as argument will be the description of the Toast
    fn error(&mut self, description: impl ToString) {
        let toast = Toast::default()
            .title(String::from("Error"))
            .color(Color::Destructive)
            .description(rsx! {
                p { "{description.to_string()}" }
            });
        self.write().toasts.push(toast);
    }

    // /// Build a toast with primary background color and title "Loading"
    // /// The string passed as argument will be the description of the Toast
    // fn loading(&mut self, description: impl ToString) {
    //     let toast = Toast::default()
    //         .title(String::from("Loading"))
    //         .color(Color::Primary)
    //         .description(rsx! {
    //             p { "{description.to_string()}" }
    //         });
    //     self.write().toasts.push(toast);
    // }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct ToasterProps {
    #[props(extends = ol, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

impl std::default::Default for ToasterProps {
    fn default() -> Self {
        Self {
            attributes: Vec::<Attribute>::default(),
            children: rsx! {},
        }
    }
}

/// The toaster must wrap around your App as high as possible to be used
#[component]
pub fn Toaster(mut props: ToasterProps) -> Element {
    props.update_class_attribute();

    let state =
        use_context_provider::<Signal<ToasterState>>(|| Signal::new(ToasterState::default()));

    rsx! {
        {props.children}
        ol { role: "alert", id: "dx-toast", ..props.attributes,
            for toast in state.read().toasts.iter() {
                ToastView { state, toast: toast.clone() }
            }
        }
    }
}

/// A Toast with a default duration of 10s
#[derive(Clone, Debug, PartialEq, UiComp)]
pub struct Toast {
    id: String,
    title: String,
    description: Element,
    duration_in_ms: u32,
    is_closable: bool,
    pub color: Color,
    pub animation: Animation,
    state: ToastState,
}

impl std::default::Default for Toast {
    fn default() -> Self {
        Self {
            id: use_unique_id(),
            title: String::default(),
            description: Ok(VNode::default()), // Default this way to be able to check the children
            duration_in_ms: 6_000,
            is_closable: true,
            color: Color::default(),
            animation: Animation::default(),
            state: ToastState::Opening,
        }
    }
}

impl Toast {
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn description(mut self, description: Element) -> Self {
        self.description = description;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    // pub fn animation(mut self, animation: Animation) -> Self {
    //     self.animation = animation;
    //     self
    // }

    // pub fn duration_in_ms(mut self, duration: u32) -> Self {
    //     self.duration_in_ms = duration;
    //     self
    // }

    // pub fn is_closable(mut self, is_closable: bool) -> Self {
    //     self.is_closable = is_closable;
    //     self
    // }
}

/// Define the state of an individual toast, used to animate the Toast
#[derive(Clone, Debug, PartialEq, Default)]
enum ToastState {
    #[default]
    Opening,
    Open,
    Closing,
    // Close is not needed since it means the Toast does not exist anymore
}

impl std::fmt::Display for ToastState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ToastState::Opening => "opening",
                ToastState::Open => "open",
                ToastState::Closing => "closing",
            }
        )
    }
}

/// Used to render the Toast, also update the ToasterState
#[component]
fn ToastView(mut state: Signal<ToasterState>, toast: ReadSignal<Toast>) -> Element {
    let class = toast.read().build_class();

    let mut toast_state = use_signal(|| ToastState::Opening);

    let duration_in_ms = toast.read().duration_in_ms;
    let toast_animation = toast.read().animation;

    // This is to animate the Toast in and out
    use_future(move || async move {
        if toast_animation != Animation::None {
            wait(10).await;
            toast_state.set(ToastState::Open);

            let animation_play_time = 150;
            wait(duration_in_ms - animation_play_time).await;

            toast_state.set(ToastState::Closing);

            wait(animation_play_time).await;
        } else {
            wait(duration_in_ms).await;
        }

        state.set(ToasterState::default());
    });

    rsx! {
        li {
            class,
            id: "{toast.read().id}",
            "data-state": toast_state.read().to_string(),
            h6 { class: "text-base font-bold", "{toast.read().title}" }
            if toast.read().is_closable {

            }
            {toast.read().description.clone()}
        }
    }
}

/// Used to add a cross mark to manually close the Toast
/// The Timeout is there to let the animation some time to play
#[component]
fn ToastClose(mut state: Signal<ToasterState>, mut toast_state: Signal<ToastState>) -> Element {
    rsx! {
        button {
            class: "absolute top-4 right-4 rounded-md transition-colors focus:outline-hidden focus:ring-3 focus:ring-foreground",
            r#type: "button",
            onclick: move |_| {
                spawn(async move {
                    toast_state.set(ToastState::Closing);
                    wait(150).await;
                });
            },
            X { class: "size-4" }
        }
    }
}

/// Hook that returns the ToasterState to spawn a Toast
pub fn use_toast() -> Signal<ToasterState> {
    // Will panic if no Toaster {} upper in the DOM
    use_context::<Signal<ToasterState>>()
}
