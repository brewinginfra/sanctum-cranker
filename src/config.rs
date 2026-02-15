use anyhow::{Context, Result};
use std::env;

#[derive(Default, Debug)]
pub struct CrankerConfig {
    pub port: u16,
    pub rpc_url: String,
    pub payer_private_key: String,
}

impl CrankerConfig {
    pub fn get_config() -> Result<Self> {
        let port = env::var("PORT")
            .context("PORT is not set")?
            .parse()
            .context("Invalid Port, Unable to parse it to u16")?;

        let rpc_url = env::var("RPC_URL").context("RPC_URL is not set")?;

        let payer_private_key =
            env::var("PAYER_PRIVATE_KEY").context("PAYER_PRIVATE_KEY is not set")?;

        Ok(Self {
            port,
            rpc_url,
            payer_private_key,
        })
    }
}