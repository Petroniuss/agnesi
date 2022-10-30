use color_eyre::Result;
use ethers::providers::Provider;
use ethers::types::{Address, U256};
use ethers::utils::{format_units, parse_ether, WEI_IN_ETHER};
use ethers_providers::{Http, JsonRpcClient, Middleware};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let provider = local_provider()?;

    let first_address = "0x5679717CE5f1c3fe5260AA513424EF5cb18569a9".parse::<Address>()?;

    let first_balance = provider.get_balance(first_address, None).await?;

    println!(
        "Wallet first address balance: {} wei = {} ETH.",
        first_balance,
        wei_to_ether(first_balance)
    );

    Ok(())
}

fn wei_to_ether(wei: U256) -> String {
    format_units(wei, "ether").unwrap()
}

fn local_provider() -> Result<Provider<Http>> {
    let endpoint = local_endpoint();
    println!("Connecting to Ganache Endpoint: {}", endpoint);

    Ok(Provider::try_from(endpoint)?.interval(Duration::from_millis(10)))
}

fn local_endpoint() -> &'static str {
    "http://127.0.0.1:7545"
}
