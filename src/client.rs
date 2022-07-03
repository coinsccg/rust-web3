#![allow(dead_code)]
#![allow(unused_imports)]

use hex_literal::hex;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr; // 当自定义类型实现FromStr时，通常采用parse进行解析，如果采用from_str会报FromStr不在作用域范围，因此需要导入
use secp256k1::SecretKey;
use ethereum_types;
use web3::{
    self,
    Web3,
    Error,
    transports,
    contract::{Contract, tokens::Tokenizable, Options, Error as ContractError},
    ethabi::{self, Contract as AbiContract, Error as AbiError},
    signing::SecretKeyRef,
    types::{
        BlockNumber,
        U64,
        H160,
        U256,
        H256,
        Address,
        TransactionParameters
    }
};

type Result<T> = std::result::Result<T, Error>;
type FileResult<T=String> = std::result::Result<T, std::io::Error>;
type ContractResult<T> = std::result::Result<T, ContractError>;

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
        let balance = self.client.eth().balance(Address::from_str(account).unwrap(), None).await?;
        Ok(balance)
    }

    pub fn load_contract(path: &str) -> FileResult<String>{
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub async fn contract_constant_call(&self, path: &str, contract: &str, func_name: &str, param: &str) -> ContractResult<U256> {
        // include_bytes!宏需要一个字符串字面值而不是变量
        let contract = Contract::from_json(
            self.client.eth(), contract.parse().unwrap(), 
            Self::load_contract(path).unwrap().as_bytes())?;
        
        let param: Address = param.parse().unwrap();
        // 因为query函数是一个async函数，返回的R类型不明确，因此必须在调用函数的时候指定类型
        let balance: U256 = contract.query(func_name, (param,), None, Options::default(), None).await.unwrap();
        Ok(balance)
    }
    pub async fn contract_sign_call(
        &self, 
        path: &str, 
        priv_key: &str,
        contract: &str, 
        func_name: &str, 
        param: (&str, u64)) -> ContractResult<H256> {
        let contract = Contract::from_json(
            self.client.eth(), 
            contract.parse().unwrap(), 
            Self::load_contract(path).unwrap().as_bytes())?;
        let private_key = SecretKey::from_str(priv_key).unwrap();
        let private_key_ref = SecretKeyRef::new(&private_key);
        let mut options = Options::default();
        options.gas = Some(U256::from(1000_00_u64));
        let tx_hash = contract.signed_call(
            func_name, 
            (param.0.parse::<Address>().unwrap(), U256::from(param.1)), 
            options, private_key_ref).await?;
        Ok(tx_hash)
    }

    pub async fn transfer_eth(&self, to: &str, priv_key: &str) -> Result<H256> {
        let to = Address::from_str(to).unwrap();
        let private_key = SecretKey::from_str(priv_key).unwrap();
        let tx_obj = TransactionParameters{
            to: Some(to),
            value: U256::exp10(15),
            ..Default::default()
        };
       let sign = self.client.accounts().sign_transaction(tx_obj, &private_key).await?;
       let result = self.client.eth().send_raw_transaction(sign.raw_transaction).await?;
       Ok(result)
    }

    pub async fn deploy(&self, account: &str, priv_key: &str) -> ContractResult<U256>{
        let bytecode = include_str!("./json/bytecode.bin").trim_end();
        let abi = include_bytes!("./json/token.json");
        // let my_account: Address = hex!("7cD1CB03FAE64CBab525C3263DBeB821Afd64483").into();
        let account: Address = account.parse().unwrap();
        let private_key = SecretKey::from_str(priv_key).unwrap();
        let private_key_ref = SecretKeyRef::new(&private_key);
        let contract = Contract::deploy(self.client.eth(), abi)?
        .confirmations(5)
        // .options(Options::with(|opt|{
            // opt.value = Some(5.into());
            // opt.gas_price = Some(5.into());
            // opt.gas = Some(3_000_000.into());
        // }))
        .sign_with_key_and_execute(
            bytecode, 
            (U256::from(1_000_000_u64), "My Token Coin".to_owned(), 3u64, "MTC".to_owned()), 
            private_key_ref,
            None
        )
        .await?;
        let balance = contract.query("balanceOf", (account,), None, Options::default(), None).await?;
        Ok(balance)
    }
}