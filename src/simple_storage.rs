use color_eyre::Result;
use ethers::types::Address;
use ethers_contract::Contract;
use crate::transaction::local_provider;


async fn simple_storage_interaction() -> Result<()> {
    let provider = local_provider();

    Ok(())
}