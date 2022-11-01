use crate::simple_storage::simple_storage_interaction;
use color_eyre::Result;
use log::LevelFilter;

mod simple_storage;
mod solidity;
mod transaction;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // {
    //     transaction::example().await?;
    // }

    // {
    //     let project =
    //         solidity::compile_solidity_project()?;
    //
    //     solidity::display_contract_info(&project)?;
    // }

    simple_storage_interaction().await?;

    Ok(())
}
