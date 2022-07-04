mod block;
mod client;

use web3;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let rpc = "https://data-seed-prebsc-1-s1.binance.org:8545/";
    let client = client::Client::new(rpc)?;

    let from = "0x7cD1CB03FAE64CBab525C3263DBeB821Afd64483";
    let priv_key = "664595eacc7d1dceb5e4038b5bae26fca4d63ab4d6a428ee7bdbf610e5f0dff2";
    let contract = "0x337610d27c682E347C9cD60BD4b3b107C9d34dDd";
    let func_name = "balanceOf";
    let param = "0x7cD1CB03FAE64CBab525C3263DBeB821Afd64483";

    // 查询余额
    let balance = client.get_balance(from).await?;
    println!("{:?}", balance);
    
    // 查询函数
    match client.contract_constant_call("./src/json/usdt.json", contract, func_name, param).await {
        Ok(hash) => println!("{:?}", hash),
        Err(err) => println!("{:?}", err)
    }

    // 调用合约函数
    match client.contract_sign_call(
        "./src/json/usdt.json", 
        priv_key, 
        contract, 
        "transfer", 
        ("0x337610d27c682E347C9cD60BD4b3b107C9d34dDd", 1_000_000_000_000_000_000)
    ).await {
        Ok(hash) => println!("{:?}", hash),
        Err(err) => println!("{:?}", err)
    }

    // 转账ETH
    // let to = "0xd3dE9c47b917baAd93F68B2c0D6dEe857D20b015";
    // let hash = client.transfer_eth(to, priv_key).await?;
    // println!("{:?}", hash);

    // 部署合约
    // let balance = client.deploy(from, priv_key).await.unwrap();
    // println!("{}", balance);
    Ok(())
}