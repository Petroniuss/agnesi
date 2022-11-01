use color_eyre::eyre::ContextCompat;
use color_eyre::Result;
use ethers::abi::Uint;
use ethers::providers::Provider;
use ethers::types::Address;
use ethers_contract::Contract;
use ethers_providers::Http;
use log::info;
use crate::core::local_provider;

use crate::solidity;

#[allow(dead_code)]
pub(crate) async fn simple_storage_interaction() -> Result<()> {
    let contract = prepare_simple_storage_contract()?;
    let address = "0x5679717CE5f1c3fe5260AA513424EF5cb18569a9".parse::<Address>()?;

    let value = contract.get().await?;
    info!("Current value: {}", value);

    info!("Increment by {}", address);
    contract.set(value.as_u32() + 1, address).await?;

    let value = contract.get().await?;
    info!("New value: {}", value);

    Ok(())
}

struct SimpleStorageContract {
    contract: Contract<Provider<Http>>,
}

impl SimpleStorageContract {
    async fn get(&self) -> Result<Uint> {
        let value: Uint = self.contract.method("get", ())?.call().await?;

        Ok(value)
    }

    async fn set(&self, v: u32, from: Address) -> Result<()> {
        let call = self
            .contract
            .method::<_, String>("set", Uint::from(v))?
            .from(from);

        let pending_tx = call.send().await?;

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

    let contract = Contract::new(
        simple_storage_address,
        contract_artifact.abi.unwrap().abi,
        provider,
    );

    Ok(SimpleStorageContract { contract })
}
