use dioxus::prelude::*;

use wingedcap::client::ReceiverStored;

use cross_clipboard::{copy_to_clipboard, paste_from_clipboard};
use cross_storage::storage_del;

use crate::storage::{get_receivers, store_receiver};

use lucide_dioxus::{ClipboardList, Plus, Trash2};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::components::{CreateReceiverDialog, ReceiverSecret};

#[component]
pub fn ReceiverView() -> Element {
    let mut is_create_receiver_dialog_open = use_signal(|| false);

    let mut stored_receivers: Signal<Option<Vec<(String, ReceiverStored)>>> = use_signal(|| None);

    let mut refetch = move || {
        if let Ok(receivers) = get_receivers() {
            stored_receivers.set(Some(receivers));
        }
    };

    use_effect(move || {
        refetch();
    });

    let handle_create = move |receiver_to_store: ReceiverStored| {
        let _ = store_receiver(receiver_to_store);

        is_create_receiver_dialog_open.set(false);

        refetch();
    };

    let handle_relabel = move |_storage_id: &str, _label: String| {};

    let handle_copy = move |receiver: &ReceiverStored| {
        let label = receiver.label.clone();
        let keys = receiver.keys.clone();
        let sets = receiver.sets.clone();

        let receiver = ReceiverStored { label, keys, sets };

        if let Ok(str) = serde_json::to_string(&receiver) {
            let _ = copy_to_clipboard(&str);
        }
    };

    let handle_upload = move || async move {
        let paste_result = paste_from_clipboard().await;

        match paste_result {
            Ok(receiver_content) => {
                let parsing_result = serde_json::from_str::<ReceiverStored>(&receiver_content);

                match parsing_result {
                    Ok(receiver) => {
                        let _ = store_receiver(receiver);
                    }

                    Err(e) => {
                        tracing::error!("error parsing receiver: {:?}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("error pasting receiver: {:?}", e);
            }
        }

        refetch();
    };

    let mut handle_remove = move |storage_id: &str| {
        let _ = storage_del(storage_id);

        refetch();
    };

    let handle_remove_all = move || async move {
        let current_receivers = stored_receivers.read().clone();

        if let Some(current_receivers) = current_receivers {
            for (id, _) in current_receivers {
                handle_remove(&id);
            }
        }
    };

    rsx! {
        Card { class: "animate-fade-in",
            CardHeader { class: "flex flex-row items-center gap-2 mb-6",
                div { class: "grow min-w-0",
                    CardTitle { class: "relative flex items-center",
                        span { "Receive" }

                        div { class: "absolute right-2.5 flex gap-1",
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "size-8 px-0",
                                onclick: move |_| async move {

                                    is_create_receiver_dialog_open.set(true);
                                },
                                Plus { class: "stroke-primary size-4" }
                            }

                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "size-8 px-0",
                                onclick: move |_| async move {
                                    handle_upload().await;
                                },
                                ClipboardList { class: "size-4" }
                            }

                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "size-8 px-0",
                                disabled: stored_receivers().unwrap_or_default().is_empty(),
                                onclick: move |_| async move {
                                    handle_remove_all().await;
                                },
                                Trash2 { class: "stroke-destructive size-4" }
                            }
                        }
                    }
                    CardDescription { "your future gifts" }
                }
            }

            if let Some(stored_receivers) = stored_receivers() {
                CardContent { class: "flex flex-col gap-3",
                    for (_ , (storage_id , stored_receiver)) in stored_receivers.iter().map(|s| s.clone()).enumerate() {
                        {
                            let id = storage_id.clone();

                            rsx! {
                                ReceiverSecret {
                                    receiver: stored_receiver.clone(),
                                    on_relabel: move |label| handle_relabel(&storage_id, label),
                                    on_copy: move |_| handle_copy(&stored_receiver),
                                    on_remove: move |_| handle_remove(&id),
                                }
                            }
                        }
                    }

                    if stored_receivers.is_empty() {
                        div { class: "rounded-lg border border-primary/40 bg-primary/5 px-5 py-4 text-sm text-primary animate-fade-in",
                            p { class: "font-semibold mb-3 text-base",
                                "No secrets to receive yet? Let's fix it!"
                            }

                            p { class: "mb-2 flex items-center gap-2",
                                Plus { class: "inline size-4 stroke-primary/70" }
                                span { "You can create one from scratch" }
                            }
                            p { class: "flex items-center gap-2",
                                ClipboardList { class: "inline size-4 stroke-primary/70" }
                                span { "Or load a backup from clipboard" }
                            }
                        }
                    }
                }
            }
        }

        CreateReceiverDialog {
            on_submit: handle_create,
            open: is_create_receiver_dialog_open(),
            on_open_change: move |open| is_create_receiver_dialog_open.set(open),
        }
    }
}
