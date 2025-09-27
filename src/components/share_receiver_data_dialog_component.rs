use dioxus::prelude::*;
use lucide_dioxus::{ClipboardCheck, Copy, TriangleAlert, X};

use crate::ui::{
    button::{Button, ButtonVariant},
    card::{CardContent, CardDescription, CardHeader, CardTitle},
    modal::{Modal, ModalBackground, ModalContent},
};

#[derive(Clone, Copy, Debug)]
pub struct ShareReceiverDataDialogState {
    _open: bool,
    open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
}

impl ShareReceiverDataDialogState {
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
pub struct ShareReceiverDataDialogProps {
    #[props(into)]
    on_copy: Callback<()>,
    #[props(optional)]
    open: Option<bool>,
    #[props(optional)]
    on_open_change: Option<Callback<bool>>,
}

#[component]
pub fn ShareReceiverDataDialog(props: ShareReceiverDataDialogProps) -> Element {
    let mut state =
        use_signal(|| ShareReceiverDataDialogState::new(props.open, props.on_open_change));

    let open = props.open;

    use_effect(use_reactive!(|open| {
        state.write().open = open;
    }));

    let mut is_copied = use_signal(|| false);

    use_effect(move || {
        if !state().is_open() {
            is_copied.set(false);
        }
    });

    let handle_copy = move |_| {
        spawn(async move {
            props.on_copy.call(());
        });

        is_copied.set(true);
    };

    rsx! {
        Modal {
            open: state().is_open(),
            onopenchange: move |open| state().set_is_open(open),

            ModalBackground { interactive: false }

            ModalContent { class: "",

                CardHeader { class: "mb-6",
                    CardTitle { "Secret Created !" }

                    CardDescription {
                        "The intended recipient should now load the reference to your secret in order to periodically check its state, and recover it once unlocked. Use the button below to copy the receiver data and send it through a secure channel."
                    }
                }

                CardContent { class: "",
                    div { class: "flex items-center gap-4 mb-6 rounded-md border border-destructive bg-destructive/10 px-4 py-2 text-sm text-destructive",
                        TriangleAlert { class: "h-4 w-4 shrink-0" }
                        span {
                            "Do NOT close before copied and secured. If you do, your secret will be lost forever !"
                        }
                    }

                    div { class: "flex items-center justify-between gap-4 w-full mt-4",
                        Button {
                            variant: ButtonVariant::Outline,
                            onclick: handle_copy,
                            disabled: is_copied(),
                            class: "flex items-center justify-between w-max gap-4",
                            if is_copied() {
                                "Data Copied"
                                ClipboardCheck { class: "h-4 w-4 shrink-0" }
                            } else {
                                "Copy Data"
                                Copy { class: "" }
                            }
                        }

                        Button {
                            variant: ButtonVariant::Destructive,
                            onclick: move |_| state().set_is_open(false),
                            disabled: !is_copied(),
                            class: "flex items-center justify-between w-max gap-4",
                            "Close"
                            X { class: "" }
                        }
                    }
                }
            }
        }
    }
}
