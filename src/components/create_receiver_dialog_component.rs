use dioxus::prelude::*;
use lucide_dioxus::{ClipboardCheck, ClipboardList, Info, Save, X};

use wingedcap::client::{Receiver, ReceiverStored};

use cross_clipboard::paste_from_clipboard;

use crate::ui::{
    button::{Button, ButtonVariant},
    card::{CardContent, CardDescription, CardHeader, CardTitle},
    hovercard::{HoverCard, HoverCardContent, HoverCardTrigger},
    input_animated_label::InputAnimatedLabel,
    modal::{Modal, ModalBackground, ModalContent},
};

#[derive(Clone, Debug)]
pub struct CreateReceiverFormData {
    pub label: String,
    pub receiver: Option<Receiver>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateReceiverDialogState {
    _open: bool,
    open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
}

impl CreateReceiverDialogState {
    fn new(open: Option<bool>, on_open_change: Option<Callback<bool>>) -> Self {
        Self {
            _open: false,
            open,
            on_open_change,
        }
    }

    fn is_open(&self) -> bool {
        match self.open {
            Some(open) => open,
            None => self._open,
        }
    }

    fn set_is_open(&mut self, open: bool) {
        match self.on_open_change {
            Some(on_open_change) => on_open_change(open),
            None => self._open = open,
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct CreateReceiverDialogProps {
    #[props(into)]
    on_submit: Callback<ReceiverStored>,
    #[props(optional)]
    open: Option<bool>,
    #[props(optional)]
    on_open_change: Option<Callback<bool>>,
}

#[component]
pub fn CreateReceiverDialog(props: CreateReceiverDialogProps) -> Element {
    let mut state = use_signal(|| CreateReceiverDialogState::new(props.open, props.on_open_change));

    let open = props.open;

    use_effect(use_reactive!(|open| {
        state.write().open = open;
    }));

    let default_form_data = CreateReceiverFormData {
        label: String::new(),
        receiver: None,
    };

    let mut form_data = use_signal(|| default_form_data.clone());

    let is_loaded = form_data.read().receiver.is_some();

    let mut reset_form = move || {
        form_data.set(default_form_data.clone());
    };

    use_effect(move || {
        if !state().is_open() {
            reset_form();
        }
    });

    let handle_paste = move |_| async move {
        let paste_result = paste_from_clipboard().await;

        match paste_result {
            Ok(receiver_content) => {
                let parsing_result = serde_json::from_str::<Receiver>(&receiver_content);

                match parsing_result {
                    Ok(pasted_receiver) => {
                        form_data.with_mut(|data| {
                            data.receiver = Some(pasted_receiver);
                        });
                    }

                    Err(e) => {
                        tracing::error!("error parsing pasted receiver: {:?}", e);
                    }
                }
            }

            Err(e) => {
                tracing::error!("error pasting pasted receiver: {:?}", e);
            }
        }
    };

    let handle_submit = move || {
        let receiver = form_data.read().receiver.clone();

        if let Some(receiver) = receiver {
            let label = form_data.read().label.clone();

            if label.is_empty() {
                return;
            }

            props.on_submit.call(ReceiverStored {
                label,
                keys: receiver.keys.clone(),
                sets: receiver.sets.clone(),
            });
        }
    };

    let is_form_invalid = form_data.read().label.is_empty() || form_data.read().receiver.is_none();

    rsx! {
        Modal {
            open: state().is_open(),
            onopenchange: move |open| state().set_is_open(open),

            ModalBackground {}

            ModalContent { class: "",
                div {
                    onclick: move |_| state().set_is_open(false),
                    class: "absolute top-2 right-2 cursor-pointer z-10",
                    X { class: "stroke-destructive h-4 w-4 stroke-3" }
                }

                CardHeader { class: "mb-6",
                    CardTitle { "Receive a Secret" }

                    CardDescription { "Give it a label and use the button below to paste the receiver data." }
                }

                CardContent { class: "w-full",
                    form { class: "w-full", onsubmit: move |_| handle_submit(),
                        div { class: "flex items-center gap-2 w-full",

                            InputAnimatedLabel {
                                label: "Label",
                                value: form_data.read().label.clone(),
                                oninput: move |e: FormEvent| form_data.with_mut(|data| data.label = e.value()),
                                container_class: "grow",
                            }

                            HoverCard {
                                HoverCardTrigger {
                                    Button {
                                        r#type: "button",
                                        variant: ButtonVariant::Ghost,
                                        class: "px-0",
                                        Info { class: "text-blue-500" }
                                    }
                                }

                                HoverCardContent { class: "max-w-50", "A short label that only you see" }
                            }
                        }

                        div { class: "flex items-center justify-between gap-4 w-full mt-4",
                            Button {
                                r#type: "button",
                                variant: ButtonVariant::Outline,
                                onclick: handle_paste,
                                disabled: is_loaded,
                                class: "flex items-center justify-between w-max gap-4",
                                if is_loaded {
                                    "Data Loaded"
                                    ClipboardCheck { class: "h-4 w-4 shrink-0" }
                                } else {
                                    "Paste Data"
                                    ClipboardList { class: "h-4 w-4 shrink-0" }
                                }
                            }

                            Button {
                                class: "flex items-center justify-between w-max gap-4",
                                disabled: is_form_invalid,
                                "Save"
                                Save { class: " h-4 w-4 shrink-0" }
                            }
                        }
                    }
                }
            }
        }
    }
}
