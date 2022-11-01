use color_eyre::Result;
use log::LevelFilter;

mod solidity;
mod transaction;
mod simple_storage;

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

    {

    }

    Ok(())
}

