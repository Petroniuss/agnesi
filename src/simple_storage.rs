use std::sync::Arc;
use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use ethers::abi::Uint;
use ethers::providers::Provider;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;
use ethers_contract::Contract;
use ethers_providers::{Http, Middleware};
use log::info;
use crate::core::{base_wallet, local_provider};

use crate::solidity;

#[allow(dead_code)]
pub(crate) async fn simple_storage_interaction() -> Result<()> {
    let contract = prepare_simple_storage_contract()?;
    let address = "0x5679717CE5f1c3fe5260AA513424EF5cb18569a9".parse::<Address>()?;

    let value = contract.get().await?;
    info!("Current value: {}", value);

    info!("Increment by {}", address);
    contract.set(value.as_u32() + 1, base_wallet()?).await?;

    let value = contract.get().await?;
    info!("New value: {}", value);

    Ok(())
}

struct SimpleStorageContract {
    client: Arc<Provider<Http>>,
    contract: Contract<Provider<Http>>,
}

impl SimpleStorageContract {
    async fn get(&self) -> Result<Uint> {
        let value: Uint = self.contract.method("get", ())?.call().await?;

        Ok(value)
    }

    async fn set(&self, v: u32, from_wallet: LocalWallet) -> Result<()> {
        let from = from_wallet.address();

        let mut call = self
            .contract
            .method::<_, String>("set", Uint::from(v))?;

        let nonce = self.client
            .get_transaction_count(from, None).await?;

        let tx = call.tx
            .set_nonce(nonce)
            .set_gas(50000)
            .set_gas_price(1)
            .set_from(from);

        let signature = from_wallet
            .sign_transaction(&tx)
            .await?;

        let pending_tx = self.client.send_raw_transaction(
            tx.rlp_signed(&signature)
        ).await?;

        let receipt = pending_tx.await?.context("missing receipt")?;

        info!("{:#?}", receipt);

        Ok(())
    }
}

fn prepare_simple_storage_contract() -> Result<SimpleStorageContract> {
    let provider = local_provider()?;
    let project = solidity::compile_solidity_project()?;

    let contract_artifact = solidity::find_contract_by_name("SimpleStorage", &project)?;

    let simple_storage_address = "0x524A613F5F13Ba8340afcF63DAF986519796F9C4".parse::<Address>()?;

    let client = Arc::new(provider);

    let contract = Contract::new(
        simple_storage_address,
        contract_artifact.abi.unwrap().abi,
        client.clone(),
    );

    Ok(SimpleStorageContract {
        client: client.clone(),
        contract
    })
}
