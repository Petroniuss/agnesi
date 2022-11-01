use std::str::FromStr;
use std::time::Duration;
use ethers::types::{Address, U256};
use ethers::utils::format_units;
use color_eyre::Result;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::{LocalWallet, Wallet, WalletError};
use ethers_providers::{Http, Provider};
use log::info;

pub(crate) fn local_provider() -> Result<Provider<Http>> {
    let endpoint = local_endpoint();
    info!("Connecting to Ganache Endpoint: {}", endpoint);

    Ok(Provider::try_from(endpoint)?.interval(Duration::from_millis(10)))
}

pub(crate) fn base_address() -> Result<Address> {
    let address = "0x5679717CE5f1c3fe5260AA513424EF5cb18569a9".parse::<Address>()?;

    Ok(address)
}

pub(crate) fn base_wallet() -> Result<Wallet<SigningKey>, WalletError> {
    LocalWallet::from_str(
        "ad24586a2544a2eec873edc81547ecfda73ff3932dde8d47a0342a6cce5a8128",
    )
}

pub(crate) fn local_endpoint() -> &'static str {
    "http://127.0.0.1:7545"
}

pub(crate) fn wei_to_ether(wei: U256) -> Result<f64> {
    let res = format_units(wei, "ether")?.parse::<f64>()?;

    Ok(res)
}
