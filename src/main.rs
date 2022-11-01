mod solidity;
mod transaction;

use color_eyre::eyre::{eyre, ContextCompat};
use color_eyre::{Report, Result};
use ethers::abi::Param;
use ethers::core::macros::ethers_contract_crate;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::{Address, TransactionRequest, U256};
use ethers::utils::format_units;
use ethers_solc::{Project, ProjectBuilder, ProjectCompileOutput, ProjectPathsConfig};
use log::{info, LevelFilter};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::builder().filter_level(LevelFilter::Info).init();

    {
        let project = solidity::compile_solidity_project()?;

        solidity::display_contract_info(&project)?;
    }

    Ok(())
}
