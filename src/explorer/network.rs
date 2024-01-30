use std::fmt::{self, Display};

use crate::Result;

const NETWORKS: [Network; 2] = [
    Network {
        name: "Ethereum mainnet",
        chain_id: 1,
        wss: "wss://ethereum.publicnode.com",
    },
    Network {
        name: "Polygon mainnet",
        chain_id: 137,
        wss: "wss://polygon-bor.publicnode.com",
    },
];

// region:			--- Networks

#[derive(Clone, Debug)]
pub struct Network {
    pub chain_id: u64,
    pub wss: &'static str,
    pub name: &'static str,
}

impl Network {
    pub fn from(chain_id: u64) -> Result<Network> {
        NETWORKS
            .into_iter()
            .find(|n| n.chain_id == chain_id)
            .ok_or("Network not available".into())
    }
}

/// Returns the list of available networks
pub fn networks() -> Vec<Network> {
    NETWORKS.to_vec()
}

// endregion:		--- Networks

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.chain_id)
    }
}
