use std::sync::Arc;

use ethers::{prelude::*};



abigen!(
    IUniswapV2Pair,
    r#"[
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#
);

#[tokio::main]
async fn main() {
    let client =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .unwrap();

    let client = Arc::new(client);

    let address = "0x0d4a11d5eeaac28ec3f61d100daf4d40471f1852"
        .parse::<Address>()
        .unwrap();
    let pair = IUniswapV2Pair::new(address, client.clone());

    let reserves = pair.get_reserves();
    let reserves = reserves.call().await.unwrap();
    println!("Reserves (ETH, USDC): {:?}", reserves);

    // .unwrap();
}
