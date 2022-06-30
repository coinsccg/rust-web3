use std::str::FromStr;

use web3::{
    self,
    Web3,
    Error,
    transports,
    types::{
        BlockNumber,
        U64,
        H160,
        U256
    }
};

pub type Result<T=()> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Client{
    pub client: Web3<transports::Http>
}

impl Client {
    pub fn new(rpc: &str) -> Result<Self> {
        let transport = web3::transports::Http::new(rpc)?;
        let web3 = web3::Web3::new(transport);
        Ok(Client {client: web3})
    }

    pub async fn get_balance(&self, account: &str) -> Result<U256> {
        // let addr = H160::from_str(account).unwrap();
        let balance = self.client.eth().balance(account.parse().unwrap(), Some(BlockNumber::Number(U64::from(1000u32)))).await?;
        Ok(balance)
    }
}