use dioxus::prelude::*;

use wingedcap::client::{ReceiverState, ReceiverStored, SenderState, SenderStored};

use lucide_dioxus::{Copy, CopyCheck, LockKeyhole, LockKeyholeOpen, Trash2};

use crate::ui::button::{Button, ButtonVariant};

use crate::ui::spinner::Spinner;

use crate::components::{RevealSecretDialog, SenderDetailsDialog};

use crate::utils::time::wait_util::wait;

#[derive(PartialEq, Props, Clone)]
pub struct SenderProps {
    pub secret: SenderStored,
    pub state: Option<SenderState>,
}

#[derive(PartialEq, Props, Clone)]
pub struct ReceiverProps {
    pub secret: ReceiverStored,
    pub state: Option<ReceiverState>,
}

#[derive(PartialEq, Clone)]
pub enum RoleProps {
    Sender(SenderProps),
    Receiver(ReceiverProps),
}

#[derive(PartialEq, Props, Clone)]
pub struct SecretProps {
    pub on_relabel: EventHandler<String>,
    pub on_copy: EventHandler<()>,
    pub on_remove: EventHandler<()>,
    pub role_props: RoleProps,
}

#[component]
pub fn Secret(
    SecretProps {
        on_relabel,
        on_copy,
        on_remove,
        role_props,
    }: SecretProps,
) -> Element {
    let mut is_just_copied = use_signal(|| false);

    let handle_copy = move |_| {
        on_copy.call(());
        is_just_copied.set(true);

        spawn(async move {
            wait(2000).await;
            is_just_copied.set(false);
        });
    };

    let label = match role_props.clone() {
        RoleProps::Sender(SenderProps { secret, .. }) => secret.label.clone(),
        RoleProps::Receiver(ReceiverProps { secret, .. }) => secret.label.clone(),
    };

    let status_icon = match role_props {
        RoleProps::Sender(SenderProps { state: None, .. })
        | RoleProps::Receiver(ReceiverProps { state: None, .. }) => rsx! {
            Spinner {}
        },

        RoleProps::Sender(SenderProps {
            state: Some(SenderState::Locked { .. }),
            ..
        })
        | RoleProps::Receiver(ReceiverProps {
            state: Some(ReceiverState::Locked { .. }),
            ..
        }) => rsx! {
            LockKeyhole { class: "text-orange-500 animate-fade-in" }
        },

        RoleProps::Sender(SenderProps {
            state: Some(SenderState::Unlocked { .. }),
            ..
        })
        | RoleProps::Receiver(ReceiverProps {
            state: Some(ReceiverState::Unlocked { .. }),
            ..
        }) => rsx! {
            LockKeyholeOpen { class: "stroke-green-500 animate-fade-in" }
        },
    };

    rsx! {
        div { class: "group flex items-center justify-between gap-2 rounded-lg border border-border/60 bg-card/50 p-2.5 hover:border-border hover:bg-card/80 hover:shadow-md transition-all duration-200",
            div { class: "flex grow items-center gap-2.5 min-w-0",
                div { class: "flex size-9 items-center justify-center shrink-0 rounded-md bg-muted/50",
                    {status_icon}
                }

                h1 { class: "flex grow items-center gap-2 text-sm font-semibold min-w-0",
                    div { class: "grid place-content-center min-w-0",
                        span { class: "truncate", "{label}" }
                    }
                }
            }

            div { class: "flex items-center gap-1 shrink-0",
                if let RoleProps::Receiver(
                    ReceiverProps { state: Some(ReceiverState::Unlocked { unlocked_sets, .. }), .. },
                ) = role_props.clone()
                {
                    {
                        let decrypted_messages: Vec<String> = unlocked_sets
                            .iter()
                            .map(|set| set.decrypted_data.clone())
                            .collect();

                        rsx! {
                            RevealSecretDialog { decrypted_messages }
                        }
                    }
                }

                if let RoleProps::Sender(SenderProps { secret, .. }) = role_props.clone() {
                    SenderDetailsDialog { secret, on_relabel }
                }

                Button {
                    variant: ButtonVariant::Ghost,
                    class: "px-0",
                    onclick: handle_copy,
                    if is_just_copied() {
                        CopyCheck { class: "" }
                    } else {
                        Copy { class: "" }
                    }
                }

                Button {
                    variant: ButtonVariant::Ghost,
                    class: "px-0",
                    onclick: move |_| on_remove.call(()),
                    Trash2 { class: "stroke-destructive" }
                }
            }
        }


    }
}
