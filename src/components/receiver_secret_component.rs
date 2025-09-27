use dioxus::prelude::*;

use wingedcap::client::{Key, Receiver, ReceiverState, ReceiverStored};

use crate::{
    components::{ReceiverProps, RoleProps, Secret},
    utils::get_secret,
};

#[derive(PartialEq, Props, Clone)]
pub struct ReceiverSecretProps {
    pub receiver: ReceiverStored,
    pub on_relabel: EventHandler<String>,
    pub on_copy: EventHandler<()>,
    pub on_remove: EventHandler<()>,
}

#[component]
pub fn ReceiverSecret(
    ReceiverSecretProps {
        receiver,
        on_relabel,
        on_copy,
        on_remove,
    }: ReceiverSecretProps,
) -> Element {
    let mut receiver_state: Signal<Option<ReceiverState>> = use_signal(|| None);

    let receiver_with_meta = receiver.clone();

    let ReceiverStored { keys, sets, .. } = receiver.clone();

    let keys: Vec<Key> = keys
        .iter()
        .map(|key| Key {
            host: key.host.clone(),
            pk: key.pk.clone(),
            id: key.id.clone(),
        })
        .collect();

    let receiver = Receiver { keys, sets };

    use_effect(use_reactive!(|receiver| {
        spawn(async move {
            if let Ok(state) = get_secret(receiver).await {
                receiver_state.set(Some(state));
            }
        });
    }));

    rsx! {
        Secret {
            on_copy,
            on_remove,
            on_relabel,
            role_props: RoleProps::Receiver(ReceiverProps {
                secret: receiver_with_meta,
                state: receiver_state.read().clone(),
            }),
        }
    }
}
