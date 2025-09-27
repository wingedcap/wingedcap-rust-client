use wingedcap::{
    client::{Key, Sender, SenderStored},
    sha_256,
};

use cross_storage::storage_set_object;

use super::super::constants::SENDER_STORAGE_NAME_PREFIX;

pub fn store_sender(sender: SenderStored) -> Result<(), String> {
    let SenderStored { keys, sets, .. } = sender.clone();

    let keys_without_meta: Vec<Key> = keys
        .iter()
        .map(|key| Key {
            host: key.host.clone(),
            pk: key.pk.clone(),
            id: key.id.clone(),
        })
        .collect();

    let sender_without_meta = Sender {
        keys: keys_without_meta,
        sets,
    };

    let sender_without_meta_json =
        serde_json::to_string(&sender_without_meta).map_err(|e| e.to_string())?;

    let storage_id = sha_256(&sender_without_meta_json);

    let storage_id = format!("{}_{}", SENDER_STORAGE_NAME_PREFIX, storage_id);

    storage_set_object(&storage_id, sender)
}
