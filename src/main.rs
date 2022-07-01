mod block;
mod client;

use web3;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let rpc = "https://data-seed-prebsc-1-s1.binance.org:8545/";
    let client = client::Client::new(rpc)?;
    let balance = client.get_balance("0x7cD1CB03FAE64CBab525C3263DBeB821Afd64483").await?;
    let from = "0x7cD1CB03FAE64CBab525C3263DBeB821Afd64483";
    let contract = "0x337610d27c682E347C9cD60BD4b3b107C9d34dDd";
    let func_name = "balanceOf";
    let param = "0x7cD1CB03FAE64CBab525C3263DBeB821Afd64483";
    println!("{:?}", balance);
    match client.contract_call("src/json/usdt.abi", from, contract, func_name, param).await {
        Ok(hash) => println!("{:?}", hash),
        Err(err) => println!("{:?}", err)
    }
    
    Ok(())
}