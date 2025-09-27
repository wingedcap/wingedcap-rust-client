use wingedcap::client::{fetch, Server, ServerWithMeta};

use wingedcap::{GetServerInput, GET_SERVER_ENDPOINT};

use super::super::{HUB_HOST, HUB_PK};

pub async fn get_server(payload: &GetServerInput) -> Result<ServerWithMeta, String> {
    let manager_server = Server {
        host: HUB_HOST.to_string(),
        pk: HUB_PK.to_string(),
    };

    fetch(&manager_server, GET_SERVER_ENDPOINT, payload).await
}
