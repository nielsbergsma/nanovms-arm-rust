use anyhow::Result;
use std::env;
use std::net::SocketAddr;

pub(crate) struct Configuration {
    pub server_address: SocketAddr,
}

pub(crate) fn from_env() -> Result<Configuration> {
    let server_address = env::var("SERVER_ADDRESS")?.parse()?;

    Ok(Configuration { server_address })
}
