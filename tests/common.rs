use std::sync::Arc;
use std::time::Duration;

use ethers::prelude::Provider;
use ethers::utils::GanacheInstance;

use ethers::providers::Http;

// connects the private key to http://localhost:8545
pub fn connect(ganache: &GanacheInstance, idx: usize) -> Arc<Provider<Http>> {
    let sender = ganache.addresses()[idx];
    let provider = Provider::<Http>::try_from(ganache.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64))
        .with_sender(sender);
    Arc::new(provider)
}
