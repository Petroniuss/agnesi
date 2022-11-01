
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use ethers::prelude::Signature;
use ethers::providers::Middleware;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::{Address, TransactionRequest};
use ethers::types::transaction::eip2718::TypedTransaction::Legacy;
use log::info;

use crate::core::{base_address, base_wallet, local_provider};
use crate::core::wei_to_ether;

#[allow(dead_code)]
pub(crate) async fn example() -> Result<()> {
    let provider = local_provider()?;

    // query existing address/wallet
    {
        let address = base_address()?;

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
        let wallet: LocalWallet = base_wallet()?;

        let address = wallet.address();

        // get current nonce.
        let nonce = provider.get_transaction_count(address, None).await?;

        let other_address = "0x700962e054A05511c87c19693AB7eF0F1d3EEA26".parse::<Address>()?;

        // Create a transaction to transfer 10000 wei to `other_address`
        let transaction_request = TransactionRequest::new()
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
