use futures::future::join_all;
use wingedcap::{
    client::{ping_key, process_sender_state, Key, Sender, SenderKeyState, SenderState, Server},
    PingKeyInput, PingKeyOutput,
};

pub async fn ping_secret(sender: Sender) -> Result<SenderState, String> {
    let Sender { keys, sets } = sender;

    let ping_futures: Vec<_> = keys
        .iter()
        .map(|key| {
            let Key { host, pk, id } = key;

            let server = Server {
                host: host.to_string(),
                pk: pk.to_string(),
            };

            let ping_input = PingKeyInput { id: id.clone() };

            async move {
                let ping_result = ping_key(&server, &ping_input).await;
                (key.clone(), ping_result)
            }
        })
        .collect();

    let ping_results = join_all(ping_futures).await;

    let keys_state: Vec<SenderKeyState> = ping_results
        .into_iter()
        .map(|(key, ping_result)| match ping_result {
            Err(_) | Ok(PingKeyOutput::Locked) => SenderKeyState::Locked(key),
            Ok(PingKeyOutput::Unlocked) => SenderKeyState::Unlocked(key),
        })
        .collect();

    let ping_secret_result = process_sender_state(keys_state, sets).await;

    ping_secret_result
}
