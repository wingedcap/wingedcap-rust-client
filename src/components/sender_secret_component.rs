use dioxus::prelude::*;

use wingedcap::client::{Key, Sender, SenderState, SenderStored};

use crate::{
    components::{RoleProps, Secret, SenderProps},
    utils::ping_secret,
};

#[derive(PartialEq, Props, Clone)]
pub struct SenderSecretProps {
    pub sender: SenderStored,
    pub on_relabel: EventHandler<String>,
    pub on_copy: EventHandler<()>,
    pub on_remove: EventHandler<()>,
}

#[component]
pub fn SenderSecret(
    SenderSecretProps {
        sender,
        on_relabel,
        on_copy,
        on_remove,
    }: SenderSecretProps,
) -> Element {
    let mut sender_state: Signal<Option<SenderState>> = use_signal(|| None);

    let sender_with_meta = sender.clone();

    let SenderStored { keys, sets, .. } = sender.clone();

    let keys: Vec<Key> = keys
        .iter()
        .map(|key| Key {
            host: key.host.clone(),
            pk: key.pk.clone(),
            id: key.id.clone(),
        })
        .collect();

    let sender = Sender { keys, sets };

    use_effect(use_reactive!(|sender| {
        spawn(async move {
            if let Ok(state) = ping_secret(sender).await {
                sender_state.set(Some(state));
            }
        });
    }));

    rsx! {
        Secret {
            on_relabel,
            on_copy,
            on_remove,
            role_props: RoleProps::Sender(SenderProps {
                secret: sender_with_meta,
                state: sender_state.read().clone(),
            }),
        }
    }
}
