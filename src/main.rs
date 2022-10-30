use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use ethers::providers::Provider;
use ethers::types::{Address, TransactionRequest, U256};
use ethers::utils::format_units;
use ethers_providers::{Http, Middleware};
use log::{info, LevelFilter};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let provider = local_provider()?;

    // query existing address/wallet
    {
        let address = "0x5679717CE5f1c3fe5260AA513424EF5cb18569a9".parse::<Address>()?;

        let balance = provider.get_balance(address, None).await?;

        info!("Wallet {} balance: {}ETH.", address, wei_to_ether(balance)?);
    }

    // query other wallet.
    {
        let address = "0x700962e054A05511c87c19693AB7eF0F1d3EEA26".parse::<Address>()?;

        let balance = provider.get_balance(address, None).await?;

        info!("Wallet {} balance: {}ETH.", address, wei_to_ether(balance)?);
    }

    // transaction
    {
        let address = "0x5679717CE5f1c3fe5260AA513424EF5cb18569a9".parse::<Address>()?;

        let other_address = "0x700962e054A05511c87c19693AB7eF0F1d3EEA26".parse::<Address>()?;

        // Create a transaction to transfer 10000 wei to `other_address`
        let tx = TransactionRequest::pay(other_address, U256::from(10000u64)).from(address);

        // Send the transaction and wait for receipt
        let receipt = provider
            .send_transaction(tx, None)
            .await?
            .await?
            .context("Missing receipt")?;

        info!("Executed Transaction: {:#?}", receipt);

        info!(
            "Balance of {} {}",
            address,
            wei_to_ether(provider.get_balance(address, None).await?)?
        );

        info!(
            "Balance of {} {}",
            other_address,
            wei_to_ether(provider.get_balance(other_address, None).await?)?
        );
    }

    Ok(())
}

fn wei_to_ether(wei: U256) -> Result<f64> {
    let res = format_units(wei, "ether")?.parse::<f64>()?;

    Ok(res)
}

fn local_provider() -> Result<Provider<Http>> {
    let endpoint = local_endpoint();
    info!("Connecting to Ganache Endpoint: {}", endpoint);

    Ok(Provider::try_from(endpoint)?.interval(Duration::from_millis(10)))
}

fn local_endpoint() -> &'static str {
    "http://127.0.0.1:7545"
}
