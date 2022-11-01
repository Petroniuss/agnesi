use std::str::FromStr;
use std::time::Duration;

use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use ethers::prelude::Signature;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest, U256};
use ethers::types::transaction::eip2718::TypedTransaction::Legacy;
use ethers::utils::format_units;
use log::info;

#[allow(dead_code)]
pub(crate) async fn example() -> Result<()> {
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
        // create a wallet with private and public key - it already exists and it has funds!
        let wallet: LocalWallet = LocalWallet::from_str(
            "ad24586a2544a2eec873edc81547ecfda73ff3932dde8d47a0342a6cce5a8128",
        )?;

        let address = wallet.address();

        // get current nonce.
        let nonce = provider.get_transaction_count(address, None).await?;

        let other_address = "0x700962e054A05511c87c19693AB7eF0F1d3EEA26".parse::<Address>()?;

        // Create a transaction to transfer 10000 wei to `other_address`
        let transaction_request =
            TransactionRequest::new()
                .to(other_address)
                .value(10000)
                .from(address)
                .gas_price(1)
                .gas(21000)
                .nonce(nonce);

        // sign the transaction
        let tx = Legacy(transaction_request);
        let signature: Signature = wallet.sign_transaction(&tx).await?;

        // Send the transaction and wait for receipt
        let receipt = provider
            .send_raw_transaction(tx.rlp_signed(&signature))
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

pub(crate) fn local_provider() -> Result<Provider<Http>> {
    let endpoint = local_endpoint();
    info!("Connecting to Ganache Endpoint: {}", endpoint);

    Ok(Provider::try_from(endpoint)?.interval(Duration::from_millis(10)))
}

fn local_endpoint() -> &'static str {
    "http://127.0.0.1:7545"
}
