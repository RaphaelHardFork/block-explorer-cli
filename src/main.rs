mod error;

use core::time;

pub use self::error::{Error, Result};

use alloy_chains::Chain;
use alloy_primitives::{
    address,
    bytes::Buf,
    hex::ToHex,
    utils::{format_ether, parse_ether},
    Address,
};
use ethers::{
    core::k256::pkcs8::der::Encode,
    etherscan::account::{self, GenesisOption},
    middleware::gas_oracle::etherscan,
    providers::{CallBuilder, Caller, Middleware, Provider, StreamExt, Ws},
    types::{transaction, BlockId, NameOrAddress, H160},
    utils::{parse_checksummed, to_checksum},
};
use foundry_block_explorers::{account::TokenQueryOption, contract, Client};
use tokio;

const POLYGON_URL: &str = "wss://polygon-bor.publicnode.com";
const ETHEREUM_URL: &str = "wss://ethereum.publicnode.com";
const API_KEY: &str = "";

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Ws>::connect(POLYGON_URL).await?;
    let explorer = Client::new(Chain::from_id(137), API_KEY)?;

    // let mut stream = provider.subscribe_blocks().await?;
    // while let Some(block) = stream.next().await {
    //     println!(
    //         "Block number: #{:?} - {:?} - {:?}",
    //         block.number.unwrap(),
    //         block.timestamp,
    //         block.hash.unwrap()
    //     );
    // }

    let contract = address!("Def1C0ded9bec7F1a1670819833240f027b25EfF");
    // let contract = contract.to_checksum_buffer(None);
    let erc20 = explorer
        .get_erc20_token_transfer_events(TokenQueryOption::ByAddress(contract), None)
        .await?;

    let balance = explorer.get_ether_balance_single(&contract, None).await?;

    println!("->> token {:?}", balance);

    // println!("->> {:?}", contract.as_slice());
    // contract.to_der();

    let a = contract.clone().to_der()?;

    let addr = NameOrAddress::Address(H160::from_slice(contract.as_slice()));

    // println!("->> {:?}", addr);

    let code = provider.get_code(addr, None).await?;
    // println!("->> Code: {:?}", code);

    let metadata = explorer.contract_abi(contract).await?;
    // println!("->> Metadata: {:?}", metadata);

    // let code = provider.get_code(contract.to_string(), None).await?;

    let addr = "0xcBe6EB3A14771f825EB02b50E8711ACB49490411";
    let addr = Address::parse_checksummed(addr, None)?;

    let txs = explorer.get_transactions(&addr, None).await?;

    for tx in txs {
        let value = tx.value;
        let value = format_ether(value);

        if let Some(to) = tx.to {
            if to == addr {
                println!("Enter {} MATIC", value);
            }
        }

        if let Some(from) = tx.from.value() {
            if from == &addr {
                println!("Out {} MATIC", value);
            }
        }

        if let Some(fn_name) = tx.function_name {
            println!("{}", fn_name);
            let to = tx.to;
            match to {
                Some(to) => {
                    // let code = provider
                    //     .get_code(H160::from_slice(to.as_slice()), None)
                    //     .await?;
                    // println!("Code interacted with: {:?}", code);
                }
                None => {}
            }
        }
    }

    Ok(())
}
