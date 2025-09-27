use wingedcap::client::ReceiverStored;

use cross_storage::storage_get_matches;

use super::super::constants::RECEIVER_STORAGE_NAME_PREFIX;

pub fn get_receivers() -> Result<Vec<(String, ReceiverStored)>, String> {
    storage_get_matches(RECEIVER_STORAGE_NAME_PREFIX)
}
