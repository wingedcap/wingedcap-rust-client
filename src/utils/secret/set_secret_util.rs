use futures::future::join_all;
use wingedcap::{
    client::{process_new_secret, set_key, KeyIndexArray, Receiver, Sender, Server},
    SetKeyInput, SetKeyOutput,
};

pub async fn set_secret(
    message: String,
    timelock: u64,
    servers: Vec<Server>,
    sets: Vec<KeyIndexArray>,
) -> Result<(Sender, Receiver), String> {
    let set_futures: Vec<_> = servers
        .iter()
        .map(|server| {
            let set_input = SetKeyInput { timelock };

            async move {
                let set_result = set_key(server, &set_input).await;
                (server.clone(), set_result)
            }
        })
        .collect();

    let set_results = join_all(set_futures).await;

    let keys_result: Result<Vec<(Server, SetKeyOutput)>, String> = set_results
        .into_iter()
        .map(|(server, set_result)| match set_result {
            Ok(set_output) => Ok((server, set_output)),
            Err(e) => Err(e),
        })
        .collect();

    let keys = keys_result?;

    let new_secret_result = process_new_secret(message, keys, sets).await;

    new_secret_result
}
