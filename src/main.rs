use std::str::FromStr;

use color_eyre::{
    eyre::{Context, Result},
    Help,
};

#[tokio::main]
async fn main() -> Result<()> {
    install_tracing();
    color_eyre::install()?;

    let api_key = std::env::var("ETHERSCAN_API_KEY")
        .wrap_err("Reading ETHERSCAN_API_KEY from env")
        .suggestion("Set ETHERSCAN_API_KEY to your Etherscan API key")?;
    let api_url = std::env::var("ETHERSCAN_API_URL");
    let chain = std::env::var("CHAIN")
        .wrap_err("Reading CHAIN from env")
        .suggestion("Set CHAIN to the chain you want to use, for example: CHAIN=polygon")?;
    let chain = ethers::core::types::Chain::from_str(&chain)
        .wrap_err("Parsing CHAIN from env")
        .suggestion("Read the docs for [ethers::core::types::Chain]")?;
    let client_builder = ethers::etherscan::Client::builder()
        .chain(chain)?
        .with_api_key(api_key);
    let client = if let Ok(api_url) = api_url {
        client_builder
            .with_api_url(api_url)
            .wrap_err("Parsing ETHERSCAN_API_URL from env")
            .suggestion("Set ETHERSCAN_API_URL to the URL of your Etherscan API endpoint")?
            .build()
            .wrap_err("Building Etherscan client")?
    } else {
        client_builder
            .build()
            .wrap_err("Building Etherscan client")?
    };
    let gas_oracle = client.gas_oracle().await;
    tracing::info!(?gas_oracle, "Gas Oracle Info");
    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().pretty().without_time().with_target(true);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
