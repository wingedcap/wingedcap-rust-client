use dioxus::prelude::*;

use lucide_dioxus::{ArrowRight, Info, KeyRound, Pencil, Save, Shield, TableOfContents, X};
use wingedcap::client::{get_vault_conf, SenderStored, ServerWithMeta, VaultConf};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::input::Input;
use crate::ui::modal::{Modal, ModalBackground, ModalContent, ModalTrigger};
use crate::ui::separator::Separator;

use crate::ui::card::{CardContent, CardDescription, CardHeader, CardTitle};

use crate::components::KeyDetails;

#[derive(PartialEq, Props, Clone)]
pub struct SenderDetailsDialogProps {
    pub secret: SenderStored,
    pub on_relabel: EventHandler<String>,
}

#[component]
pub fn SenderDetailsDialog(
    SenderDetailsDialogProps { secret, on_relabel }: SenderDetailsDialogProps,
) -> Element {
    let mut edited_label: Signal<Option<String>> = use_signal(|| None);

    let label = secret.label;

    let keys = secret.keys;

    let sets = secret.sets.iter().map(|keys| keys.clone()).collect();

    let num_keys = keys.len();

    let vault_conf = get_vault_conf(sets);

    let vault_conf_description = match vault_conf {
        VaultConf::Standard { total, required } => {
            format!("Standard {required} of {total}")
        }
        VaultConf::Custom => "Custom".to_string(),
    };

    let label_for_render = label.clone();

    let label_for_handle_enable_editing = label.clone();

    let handle_enable_editing_lable = move || {
        edited_label.set(Some(label_for_handle_enable_editing.clone()));
    };

    let handle_label_input_change = move |current_label: String| {
        edited_label.set(Some(current_label));
    };

    let handle_cancel_editing_label = move || {
        edited_label.set(None);
    };

    let handle_save_label = move |new_label: String| {
        on_relabel(new_label);

        edited_label.set(None);
    };

    rsx! {
        Modal {
            ModalTrigger { class: "px-0 pr-0 flex items-center justify-center border-none shadow-none",
                Info { class: "stroke-primary" }
            }

            ModalBackground {}

            ModalContent {
                CardHeader { class: "",
                    CardTitle { class: "",
                        form {
                            class: "flex items-center gap-1 relative",
                            onsubmit: move |_| {
                                if let Some(current_value) = edited_label() {
                                    let mut handle_save_label = handle_save_label.clone();
                                    handle_save_label(current_value.clone())
                                }
                            },
                            onreset: move |e: FormEvent| {
                                e.prevent_default();
                                let mut handle_cancel_editing_label = handle_cancel_editing_label.clone();
                                handle_cancel_editing_label()
                            },

                            div { class: "flex items-center gap-1 relative",
                                Input {
                                    class: "text-xl pl-1 -ml-1 disabled:opacity-100 disabled:cursor-text disabled:border-transparent",
                                    value: edited_label().unwrap_or(label),
                                    disabled: edited_label().is_none(),
                                    oninput: move |e: FormEvent| {
                                        let mut handle_label_input_change = handle_label_input_change.clone();

                                        handle_label_input_change(e.value());
                                    },
                                }

                                if edited_label().is_some() {
                                    Button {
                                        r#type: "reset",
                                        variant: ButtonVariant::Ghost,
                                        class: "px-0 absolute right-1.5 size-7",
                                        X { class: "stroke-destructive" }
                                    }
                                }
                            }

                            if edited_label().is_none() {
                                Button {
                                    r#type: "button",
                                    variant: ButtonVariant::Ghost,
                                    class: "px-0 shrink-0",
                                    onclick: move |_| {
                                        let mut handle_enable_editing_lable = handle_enable_editing_lable.clone();
                                        handle_enable_editing_lable()
                                    },
                                    Pencil { class: "" }
                                }
                            }

                            if let Some(current_value) = edited_label() {
                                Button {
                                    r#type: "submit",
                                    variant: ButtonVariant::Ghost,
                                    class: "px-0 shrink-0",
                                    disabled: current_value == label_for_render,
                                    Save { class: "stroke-primary" }
                                }
                            }
                        }
                    

                    }
                    CardDescription { "Detailed configuration and management" }
                }

                CardContent {
                    div { class: "",
                        div { class: "relative mt-8 mb-5 flex items-center justify-center",
                            Separator { class: "w-full" }

                            div { class: "text-muted-foreground bg-background absolute flex items-center gap-2 px-4 text-sm font-semibold",
                                span { class: "tracking-wider", "General" }
                                TableOfContents { class: "" }
                            }
                        }

                        div { class: "grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 w-max *:odd:font-medium text-sm",
                            span { class: "", "No. keys" }
                            span { class: "", "{num_keys} (see below)" }

                            span { class: "", "Vault type" }
                            span { class: "", "{vault_conf_description}" }
                        }
                    }

                    div { class: "",
                        div { class: "relative mt-8 mb-5 flex items-center justify-center",
                            Separator { class: "w-full" }

                            div { class: "text-muted-foreground bg-background absolute flex items-center gap-2 px-4 text-sm font-semibold",
                                span { class: "tracking-wider", "Keys" }
                                KeyRound { class: "" }
                            }
                        }

                        div { class: "space-y-1 w-max text-sm items-center",
                            for (key_index , key) in keys.iter().enumerate() {
                                div { class: "flex items-center gap-5",
                                    div { class: "grid items-center justify-center grid-cols-1",
                                        span { class: "row-start-1 col-start-1 text-center text-xs font-semibold",
                                            "{key_index + 1}"
                                        }
                                        Shield { class: "row-start-1 col-start-1" }
                                    }

                                    ArrowRight { class: "" }

                                    div { class: "border border-border border-dashed rounded-md p-0.5",
                                        KeyDetails {
                                            key_with_meta: ServerWithMeta {
                                                host: key.host.clone(),
                                                pk: key.pk.clone(),
                                                meta: key.meta.clone(),
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
