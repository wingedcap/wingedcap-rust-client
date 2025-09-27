use wingedcap::client::SenderStored;

use cross_storage::storage_get_matches;

use super::super::constants::SENDER_STORAGE_NAME_PREFIX;

pub fn get_senders() -> Result<Vec<(String, SenderStored)>, String> {
    storage_get_matches(SENDER_STORAGE_NAME_PREFIX)
}
