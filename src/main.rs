use color_eyre::Result;
use log::LevelFilter;

mod solidity;
mod transaction;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // {
    //     let project = solidity::compile_solidity_project()?;
    //
    //     solidity::display_contract_info(&project)?;
    // }
    {
        transaction::example().await?;
    }

    Ok(())
}
