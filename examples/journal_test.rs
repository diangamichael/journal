//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use eyre::eyre;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

/// Your private key file path.
const PRIV_KEY_PATH: &str = "PRIV_KEY_PATH";

/// Stylus RPC endpoint url.
const RPC_URL: &str = "RPC_URL";

/// Deployed pragram address.
const STYLUS_PROGRAM_ADDRESS: &str = "STYLUS_PROGRAM_ADDRESS";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let priv_key_path =
        std::env::var(PRIV_KEY_PATH).map_err(|_| eyre!("No {} env var set", PRIV_KEY_PATH))?;
    let rpc_url = std::env::var(RPC_URL).map_err(|_| eyre!("No {} env var set", RPC_URL))?;
    let program_address = std::env::var(STYLUS_PROGRAM_ADDRESS)
        .map_err(|_| eyre!("No {} env var set", STYLUS_PROGRAM_ADDRESS))?;
    abigen!(
        Journal,
        r#"[
            function newEntry(string calldata title, string calldata body) external
            function getEntry(address _address, uint256 idx) external view returns (uint8[] memory)
        ]"#
    );

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let address: Address = program_address.parse()?;

    let privkey = read_secret_from_file(&priv_key_path)?;
    let wallet = LocalWallet::from_str(&privkey)?;
    let user_address = wallet.address();
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let journal = Journal::new(address, client);

    journal
        .new_entry("test entry".to_string(), "body body body".to_string())
        .send()
        .await?
        .await?;
    println!("New entry added to the journal");

    let entry = journal.get_entry(user_address, U256::from(1)).await?;
    println!("Journal Body = {:?}", entry);
    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}
