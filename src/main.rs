mod block;
mod client;

use web3;

#[tokio::main]
async fn main() -> web3::Result<()> {
    let rpc = "https://data-seed-prebsc-1-s1.binance.org:8545/";
    let cc = client::Client::new(rpc)?;
    let balance = cc.get_balance("0x7cD1CB03FAE64CBab525C3263DBeB821Afd64483").await?;
    println!("{:?}", balance);
    Ok(())
}