use wingedcap::utils::wait as sdk_wait;

pub async fn wait(ms: u32) {
    sdk_wait(ms).await;
}
