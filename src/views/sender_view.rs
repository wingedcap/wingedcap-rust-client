use dioxus::prelude::*;

use wingedcap::client::{
    generate_standard_vault_sets, KeyWithMeta, Receiver, SenderStored, Server, ServerWithMeta,
};

use crate::types::Time;

use cross_clipboard::{copy_to_clipboard, paste_from_clipboard};
use cross_storage::{storage_del, storage_set};

use crate::storage::{get_senders, store_sender};

use crate::utils::{set_secret, time_to_seconds};

use lucide_dioxus::{ClipboardList, Plus, Trash2};

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use crate::components::{CreateSecretFormDialog, SenderSecret, ShareReceiverDataDialog};

#[derive(Clone, Debug)]
pub struct CreateSecretFormData {
    pub label: String,
    pub message: String,
    pub timelock: Time,
    pub servers: Vec<ServerWithMeta>,
    pub required_keys: u64,
}

#[component]
pub fn SenderView() -> Element {
    let mut new_secret_receiver: Signal<Option<Receiver>> = use_signal(|| None);

    let mut is_create_dialog_open = use_signal(|| false);
    let mut is_share_receiver_data_dialog_open = use_signal(|| false);

    let mut stored_senders: Signal<Option<Vec<(String, SenderStored)>>> = use_signal(|| None);

    let mut refetch = move || {
        if let Ok(senders) = get_senders() {
            stored_senders.set(Some(senders));
        }
    };

    use_effect(move || {
        refetch();
    });

    use_effect(move || {
        if !is_share_receiver_data_dialog_open() {
            new_secret_receiver.set(None);
        }
    });

    let handle_create = move |CreateSecretFormData {
                                  label,
                                  message,
                                  timelock,
                                  servers: servers_with_meta,
                                  required_keys,
                              }: CreateSecretFormData| async move {
        let timelock = time_to_seconds(timelock);

        let servers: Vec<Server> = servers_with_meta
            .iter()
            .map(|ServerWithMeta { host, pk, .. }| Server {
                host: host.clone(),
                pk: pk.clone(),
            })
            .collect();

        let sets = generate_standard_vault_sets(servers.len() as u64, required_keys);

        let new_secret_result = set_secret(message, timelock, servers, sets).await;

        match new_secret_result {
            Ok((sender, receiver)) => {
                let keys_with_meta = sender
                    .keys
                    .iter()
                    .enumerate()
                    .map(|(index, key)| {
                        let ServerWithMeta { meta, .. } = servers_with_meta[index].clone();

                        KeyWithMeta {
                            host: key.host.clone(),
                            pk: key.pk.clone(),
                            id: key.id.clone(),
                            meta: meta.clone(),
                        }
                    })
                    .collect();

                let sender_to_store = SenderStored {
                    label: label.clone(),
                    keys: keys_with_meta,
                    sets: sender.sets.clone(),
                };

                let _ = store_sender(sender_to_store);

                is_create_dialog_open.set(false);

                new_secret_receiver.set(Some(receiver));

                is_share_receiver_data_dialog_open.set(true);

                refetch();

                Ok(())
            }

            Err(e) => {
                let error = format!("error creating sender: {:?}", e);
                tracing::error!(error);
                Err(error)
            }
        }
    };

    let mut handle_relabel = move |storage_id: &str, label: String| {
        let label_clone = label.clone();

        if let Some(stored_senders) = stored_senders() {
            for (id, sender) in stored_senders {
                let label = label_clone.clone();

                if id == storage_id {
                    let new_data = SenderStored { label, ..sender };

                    match serde_json::to_string(&new_data) {
                        Ok(new_data_json) => match storage_set(&storage_id, &new_data_json) {
                            Ok(_) => {
                                refetch();
                            }

                            Err(e) => {
                                tracing::error!("error updating sender in storage: {:?}", e);
                            }
                        },
                        Err(e) => {
                            tracing::error!("error serializing sender: {:?}", e);
                        }
                    }

                    break;
                }
            }
        }
    };

    let handle_copy_receiver = move || {
        let receiver = new_secret_receiver.read().clone();

        if let Ok(str) = serde_json::to_string_pretty(&receiver) {
            let _ = copy_to_clipboard(&str);

            is_create_dialog_open.set(false);
        }
    };

    let handle_copy = move |sender: &SenderStored| {
        if let Ok(str) = serde_json::to_string(&sender) {
            let _ = copy_to_clipboard(&str);
        }
    };

    let handle_upload = move || async move {
        let paste_result = paste_from_clipboard().await;

        match paste_result {
            Ok(sender_content) => {
                let parsing_result = serde_json::from_str::<SenderStored>(&sender_content);

                match parsing_result {
                    Ok(sender) => {
                        let _ = store_sender(sender);
                    }

                    Err(e) => {
                        tracing::error!("error parsing sender: {:?}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("error pasting sender: {:?}", e);
            }
        }

        refetch();
    };

    let mut handle_remove = move |storage_id: &str| {
        let _ = storage_del(storage_id);

        refetch();
    };

    let handle_remove_all = move || async move {
        let current_senders = stored_senders.read().clone();

        if let Some(current_senders) = current_senders {
            for (id, _) in current_senders {
                handle_remove(&id);
            }
        }
    };

    rsx! {
        Card { class: "animate-fade-in",
            CardHeader { class: "flex flex-row items-center gap-2 mb-6",
                div { class: "grow min-w-0",
                    CardTitle { class: "relative flex items-center",
                        span { "Send" }

                        div { class: "absolute font-normal right-2.5 flex gap-1",
                            CreateSecretFormDialog {
                                on_submit: move |(data, callback): (CreateSecretFormData, Callback<Result<(), String>>)| async move {
                                    callback(handle_create(data).await);
                                },
                                open: is_create_dialog_open(),
                                on_open_change: move |open| is_create_dialog_open.set(open),
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
                                disabled: stored_senders().unwrap_or_default().is_empty(),
                                onclick: move |_| async move {
                                    handle_remove_all().await;
                                },
                                Trash2 { class: "stroke-destructive size-4" }
                            }
                        }
                    }
                    CardDescription { "your future echoes" }
                }
            }

            if let Some(stored_senders) = stored_senders() {
                CardContent { class: "flex flex-col gap-3",
                    for (_ , (storage_id , stored_sender)) in stored_senders.iter().map(|s| s.clone()).enumerate() {
                        {
                            let id = storage_id.clone();

                            rsx! {
                                SenderSecret {
                                    sender: stored_sender.clone(),
                                    on_relabel: move |label| handle_relabel(&id, label),
                                    on_copy: move |_| handle_copy(&stored_sender),
                                    on_remove: move |_| handle_remove(&storage_id),
                                }
                            }
                        }
                    }

                    if stored_senders.is_empty() {
                        div { class: "rounded-lg border border-primary/40 bg-primary/5 px-5 py-4 text-sm text-primary animate-fade-in",
                            p { class: "font-semibold mb-3 text-base",
                                "No secrets to send yet? Let's fix it!"
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

        ShareReceiverDataDialog {
            on_copy: handle_copy_receiver,
            open: is_share_receiver_data_dialog_open(),
            on_open_change: move |open| is_share_receiver_data_dialog_open.set(open),
        }
    }
}
