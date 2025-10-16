use std::cmp::{max, min};

use strum::IntoEnumIterator;

use dioxus::prelude::*;

use lucide_dioxus::{ClipboardList, Info, KeyRound, Plus, SquarePen, Trash2};

use wingedcap::{client::ServerWithMeta, GetServerInput};

use cross_clipboard::paste_from_clipboard;

use crate::components::KeyDetails;
use crate::types::{Time, TimeUnit};

use crate::ui::toast::{use_toast, ToastRenderer};
use crate::utils::{get_time_unit_from_name, get_time_unit_name};

use crate::manager::get_server;

use crate::ui::select::SelectValue;

use crate::ui::{
    button::{Button, ButtonVariant},
    card::{CardDescription, CardHeader, CardTitle},
    hovercard::{HoverCard, HoverCardContent, HoverCardTrigger},
    input_animated_label::InputAnimatedLabel,
    modal::{Modal, ModalBackground, ModalContent, ModalTrigger},
    select::{Select, SelectContent, SelectItem, SelectTrigger},
    separator::Separator,
    spinner::Spinner,
};

use crate::views::sender_view::CreateSecretFormData;

#[derive(Clone, Copy, Debug)]
pub struct CreateSecretFormDialogState {
    _open: bool,
    open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
}

impl CreateSecretFormDialogState {
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
pub struct CreateSecretFormDialogProps {
    #[props(into)]
    on_submit: Callback<(CreateSecretFormData, Callback<Result<(), String>>)>,
    #[props(optional)]
    open: Option<bool>,
    #[props(optional)]
    on_open_change: Option<Callback<bool>>,
}

#[component]
pub fn CreateSecretFormDialog(props: CreateSecretFormDialogProps) -> Element {
    let mut toast = use_toast();

    let mut state =
        use_signal(|| CreateSecretFormDialogState::new(props.open, props.on_open_change));

    let open = props.open;

    use_effect(use_reactive!(|open| {
        state.write().open = open;
    }));

    let mut is_submitting = use_signal(|| false);
    let mut is_adding_server = use_signal(|| false);

    let default_form_data = CreateSecretFormData {
        label: String::new(),
        message: String::new(),
        timelock: Time {
            magnitude: 10,
            unit: TimeUnit::Second,
        },
        servers: vec![],
        required_keys: 1,
    };

    let mut form_data = use_signal(|| default_form_data.clone());

    let mut reset_form = move || {
        form_data.set(default_form_data.clone());
    };

    use_effect(move || {
        if !state().is_open() {
            is_submitting.set(false);

            reset_form();
        }
    });

    let mut handle_add_server = move |server: ServerWithMeta| {
        form_data.with_mut(|data| {
            data.servers.push(server);
        });
    };

    let handle_add_server_from_hub = move || {
        spawn(async move {
            is_adding_server.set(true);

            let server = get_server(&GetServerInput {}).await;

            if let Ok(server) = server {
                handle_add_server(server);
            }

            is_adding_server.set(false);
        });
    };

    let handle_add_server_from_clipboard = move || {
        spawn(async move {
            is_adding_server.set(true);

            if let Ok(server_json) = paste_from_clipboard().await {
                let server = serde_json::from_str::<ServerWithMeta>(&server_json);

                if let Ok(server) = server {
                    handle_add_server(server);
                }
            }

            is_adding_server.set(false);
        });
    };

    let mut handle_remove_server = move |index: usize| {
        form_data.with_mut(|data| {
            data.servers.remove(index);
            data.required_keys = max(1, min(data.required_keys, data.servers.len() as u64));
        });
    };

    let handle_submitted = move |result: Result<(), String>| {
        is_submitting.set(false);

        if result.is_err() {
            toast.error("Failed to create secret");
        }
    };

    let handle_submit = move || async move {
        is_submitting.set(true);

        let form_data = form_data.read().clone();

        props
            .on_submit
            .call((form_data, Callback::new(handle_submitted)));
    };

    let is_form_invalid = form_data().label.is_empty()
        || form_data().message.is_empty()
        || form_data().timelock.magnitude == 0
        || form_data().servers.is_empty()
        || form_data().required_keys == 0
        || form_data().required_keys > form_data().servers.len() as u64;

    rsx! {
        Modal {
            open: state().is_open(),
            onopenchange: move |open| state().set_is_open(open),
            ModalTrigger {
                Button { variant: ButtonVariant::Ghost, class: "px-0",
                    Plus { class: "stroke-primary" }
                }
            }

            ModalBackground {}

            ModalContent { class: "md:min-w-lg",
                CardHeader { class: "mb-6",
                    CardTitle { "Create a Secret" }
                    CardDescription { "Enter the message and key configuration" }
                }

                form {
                    class: "flex flex-col gap-4",
                    onsubmit: move |e| {
                        let handle_submit = handle_submit.clone();

                        async move {
                            e.prevent_default();
                            handle_submit().await
                        }
                    },

                    div { class: "@container flex flex-col gap-4 z-50",

                        div { class: "flex flex-col gap-x-4 gap-y-4 @md:flex-row justify-between",

                            // Label
                            div { class: "flex items-center gap-2",
                                InputAnimatedLabel {
                                    label: "Label",
                                    container_class: "grow",
                                    value: "{form_data().label}",
                                    oninput: move |e: FormEvent| {
                                        form_data.with_mut(|data| data.label = e.value());
                                    },
                                }

                                HoverCard {
                                    HoverCardTrigger {
                                        Info { class: "text-blue-500" }
                                    }

                                    HoverCardContent { class: "",
                                        "A short title just for you to keep track. No one else will see it."
                                    }
                                }
                            }

                            div { class: "flex flex-col gap-x-2 gap-y-4 @2xs:flex-row @2xs:items-center @2xs:self-auto",

                                div { class: "flex grow items-center gap-2",

                                    // Timelock
                                    div { class: "relative flex items-center",
                                        InputAnimatedLabel {
                                            label: "Lock",
                                            value: "{form_data().timelock.magnitude}",
                                            class: "w-42 pr-32 text-right",
                                            oninput: move |e: FormEvent| {
                                                form_data
                                                    .with_mut(|data| {
                                                        if let Ok(magnitude) = e.value().parse() {
                                                            data.timelock.magnitude = magnitude;
                                                        }
                                                    });
                                            },
                                        }

                                        Select {
                                            class: "absolute right-1 h-6 w-28",
                                            value: "{get_time_unit_name(form_data().timelock.unit)}",
                                            on_value_change: move |value: String| {
                                                form_data
                                                    .with_mut(|data| {
                                                        if let Ok(unit) = get_time_unit_from_name(value) {
                                                            data.timelock.unit = unit;
                                                        }
                                                    });
                                            },

                                            SelectTrigger { SelectValue {} }
                                            SelectContent { class: "",
                                                for unit in TimeUnit::iter().map(|unit| get_time_unit_name(unit)) {
                                                    SelectItem { value: "{unit}",
                                                        span { "{unit}" }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    Separator { class: "w-auto grow @md:hidden" }

                                    HoverCard {
                                        HoverCardTrigger {
                                            Info { class: "text-blue-500" }
                                        }

                                        HoverCardContent { class: "max-w-50",
                                            "The time it takes your secret to unlock after your last ping (last time you opened the app)"
                                        }
                                    }
                                }
                            }
                        }

                        // Message
                        InputAnimatedLabel {
                            label: "Message",
                            value: "{form_data().message}",
                            oninput: move |e: FormEvent| {
                                form_data.with_mut(|data| data.message = e.value());
                            },
                        }
                    }

                    div { class: "",
                        div { class: "relative my-5 flex items-center justify-center",
                            Separator { class: "w-full" }

                            div { class: "text-muted-foreground bg-background absolute flex items-center gap-2 px-4 text-sm font-semibold",
                                span { class: "tracking-wider", "Keys" }

                                HoverCard {
                                    HoverCardTrigger {
                                        Info { class: "text-blue-500" }
                                    }

                                    HoverCardContent { class: "max-w-50",
                                        "What keeps your secret locked while you're around. You need at least one, and more is generally better, as long as you don't set too few or too many required keys (see below)."
                                    }
                                }
                            }
                        }

                        div { class: "flex flex-col gap-2",
                            div {
                                id: "servers-scroll-area",
                                class: "max-h-[200px]",
                                div { id: "servers-scroll-area-viewport",
                                    div { class: "flex flex-col gap-2",
                                        for (index , server) in form_data().servers.iter().enumerate() {
                                            div { class: "flex gap-2",
                                                div { class: "border border-border rounded-md grow flex items-center h-9 px-1",
                                                    KeyDetails { key_with_meta: server.clone() }
                                                }

                                                Button {
                                                    r#type: "button",
                                                    variant: ButtonVariant::Ghost,
                                                    class: "shrink-0 px-0",
                                                    onclick: move |_| handle_remove_server(index),
                                                    Trash2 { class: "text-destructive" }
                                                }
                                            }
                                        }

                                        if is_adding_server() {
                                            div { class: "flex gap-2",
                                                div { class: "relative flex h-9 grow items-center justify-center gap-1 border-dashed px-1 font-normal border border-border rounded-md",
                                                    Spinner { class: "animate-spin" }
                                                }

                                                Button {
                                                    variant: ButtonVariant::Ghost,
                                                    class: "px-0",
                                                    onclick: move |_| {
                                                        handle_add_server_from_clipboard();
                                                    },
                                                    Trash2 { class: "text-destructive" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "flex gap-2",
                                div { class: "relative flex h-9 grow items-center justify-between gap-1 border-dashed px-1 font-normal border border-border rounded-md",

                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        class: "h-6 shrink-0 grow justify-start gap-3 px-2 rounded-sm",
                                        r#type: "button",
                                        onclick: move |_| {
                                            handle_add_server_from_hub();
                                        },
                                        Plus { class: "" }
                                        span { "Add key" }
                                    }
                                }

                                Button {
                                    r#type: "button",
                                    variant: ButtonVariant::Ghost,
                                    class: "px-0",
                                    onclick: move |_| {
                                        handle_add_server_from_clipboard();
                                    },
                                    ClipboardList { class: "" }
                                }
                            }
                        }

                        // Required Keys
                        div { class: "flex grow items-center gap-2 mt-4",
                            div { class: "flex items-center gap-2",
                                label { class: "text-muted-foreground mb-0 w-max grow text-sm text-nowrap",
                                    "Required keys"
                                }

                                Select {
                                    class: "",
                                    value: form_data().required_keys,
                                    on_value_change: move |value: String| {
                                        form_data
                                            .with_mut(|data| {
                                                if let Ok(value) = value.parse() {
                                                    data.required_keys = value;
                                                }
                                            });
                                    },

                                    SelectTrigger {
                                        div { class: "flex items-center grow justify-between gap-1.5",
                                            span { class: "text-right grow",
                                                SelectValue { class: "" }
                                                " of "
                                                "{max(1, form_data().servers.len() as u64)}"
                                            }
                                            KeyRound { class: "" }
                                        }
                                    }

                                    SelectContent { class: "",
                                        for value in 1..=max(1, form_data().servers.len() as u64) {
                                            SelectItem { class: "", value: "{value}",
                                                div { class: "flex items-center grow justify-between gap-1.5",
                                                    span { class: "text-right grow",
                                                        "{value} of {max(1, form_data().servers.len() as u64)}"
                                                    }
                                                    KeyRound { class: "" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            Separator { class: "w-auto grow" } // Separator

                            HoverCard {
                                HoverCardTrigger {
                                    Info { class: "text-blue-500" }
                                }
                                HoverCardContent { class: "max-w-50",
                                    "The minimum number of keys required to decrypt your secret"
                                }
                            }
                        }
                    }

                    Button { class: "mt-4 w-full gap-4", disabled: is_form_invalid,
                        if is_submitting() {
                            Spinner { class: "animate-spin" }
                        } else {
                            "Submit"
                            SquarePen { class: "" }
                        }
                    }
                }
            }
        }
    }
}
