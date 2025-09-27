use wingedcap::client::ReceiverStored;

use cross_storage::storage_set_object;

use super::super::constants::RECEIVER_STORAGE_NAME_PREFIX;

pub fn store_receiver(receiver: ReceiverStored) -> Result<(), String> {
    let storage_id = format!("{}_{}", RECEIVER_STORAGE_NAME_PREFIX, receiver.label);

    storage_set_object(&storage_id, receiver)
}
