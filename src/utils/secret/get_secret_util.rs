use futures::future::join_all;

use wingedcap::{
    client::{
        get_key, process_receiver_state, Key, Receiver, ReceiverKeyState, ReceiverKeyStateUnlocked,
        ReceiverState, Server,
    },
    GetKeyInput, GetKeyOutput, GetKeyOutputUnlocked,
};

pub async fn get_secret(receiver: Receiver) -> Result<ReceiverState, String> {
    let Receiver { keys, sets } = receiver;

    let get_futures: Vec<_> = keys
        .iter()
        .map(|key| {
            let Key { host, pk, id } = key;

            let server = Server {
                host: host.to_string(),
                pk: pk.to_string(),
            };

            let get_input = GetKeyInput { id: id.clone() };

            async move {
                let get_result = get_key(&server, &get_input).await;
                (key.clone(), get_result)
            }
        })
        .collect();

    let get_results = join_all(get_futures).await;

    let keys_state: Vec<ReceiverKeyState> = get_results
        .into_iter()
        .map(|(key, get_result)| {
            let Key { host, pk, id } = key;

            match get_result {
                Err(_) | Ok(GetKeyOutput::Locked) => ReceiverKeyState::Locked(Key { host, pk, id }),
                Ok(GetKeyOutput::Unlocked(GetKeyOutputUnlocked { key })) => {
                    ReceiverKeyState::Unlocked(ReceiverKeyStateUnlocked { host, pk, id, key })
                }
            }
        })
        .collect();

    let receiver_state_result = process_receiver_state(keys_state, sets).await;

    receiver_state_result
}
