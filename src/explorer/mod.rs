pub mod network;

use self::network::Network;
use crate::Result;
use alloy_chains::Chain;
use ethers::providers::{Provider, Ws};
use foundry_block_explorers::Client;

// region:			--- Explorer

pub struct Explorer {
    pub provider: Provider<Ws>,
    pub explorer: Client,
    pub network: Network,
}

impl Explorer {
    pub async fn new(explorer_api_key: &str, chain_id: u64) -> Result<Self> {
        let network = Network::from(chain_id)?;
        let provider = Provider::<Ws>::connect(network.wss).await?;
        let explorer = Client::new(Chain::from_id(chain_id), explorer_api_key)?;

        Ok(Self {
            provider,
            explorer,
            network,
        })
    }
}

// endregion:		--- Explorer
