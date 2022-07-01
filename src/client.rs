#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::prelude::*;
use web3::{
    self,
    Web3,
    Error,
    transports,
    contract::{Contract, tokens::Tokenizable, Options, Error as ContractError},
    ethabi::{self, Contract as AbiContract, Error as AbiError},
    types::{
        BlockNumber,
        U64,
        H160,
        U256,
        H256
    }
};

type Result<T> = std::result::Result<T, Error>;
type FileResult<T=String> = std::result::Result<T, std::io::Error>;
type CallResult<T> = std::result::Result<T, ContractError>;

static ZERO_ADDRESS: &'static str = "0x0000000000000000000000000000000000000000";

#[derive(Debug, Clone)]
pub struct Client{
    client: Web3<transports::Http>
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

    pub fn load_contract(path: &str) -> FileResult<String>{
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn zero_address() -> H160 {
        ZERO_ADDRESS.parse().unwrap()
    }

    pub async fn contract_call(&self, path: &str, from: &str, contract_addr: &str, func_name: &str, param: &str) -> CallResult<H256> {
        let contract = Contract::from_json(self.client.eth(), contract_addr.parse().unwrap(), Self::load_contract(path).unwrap().as_bytes())?;
        let mut options = Options::default();
        options.gas = Some(U256::from(100000u64));
        let hash = contract.call(func_name, param.to_string().into_token(), from.parse().unwrap(), options).await?;
        Ok(hash)
    }
}