use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, FunctionCall},
    macros::{felt, selector},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::{thread::sleep, time::Duration};
use url::Url;

#[tokio::main]
async fn main() {
    // Set the polling rate
    let polling_rate = 1;

    // ETH-USDC pool
    let swap_pool = felt!("0x04d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a");

    let rpc_client = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-mainnet.public.blastapi.io").unwrap(),
    ));

    let x_token = rpc_client
        .call(
            FunctionCall {
                contract_address: swap_pool,
                entry_point_selector: selector!("token0"),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap()[0];

    let x_token_decimals = rpc_client
        .call(
            FunctionCall {
                contract_address: x_token,
                entry_point_selector: selector!("decimals"),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap()[0];

    let y_token = rpc_client
        .call(
            FunctionCall {
                contract_address: swap_pool,
                entry_point_selector: selector!("token1"),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap()[0];

    let y_token_decimals = rpc_client
        .call(
            FunctionCall {
                contract_address: y_token,
                entry_point_selector: selector!("decimals"),
                calldata: vec![],
            },
            BlockId::Tag(BlockTag::Latest),
        )
        .await
        .unwrap()[0];

    loop {
        let reserves = rpc_client
            .call(
                FunctionCall {
                    contract_address: swap_pool,
                    entry_point_selector: selector!("get_reserves"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap();

        let x = reserves[0] + reserves[1];
        let y = reserves[2] + reserves[3];

        let klast = rpc_client
            .call(
                FunctionCall {
                    contract_address: swap_pool,
                    entry_point_selector: selector!("klast"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap();

        let k = klast[0] + klast[1];

        let mut x_unit = FieldElement::from(10u64);
        for _ in 0..x_token_decimals.to_string().parse().unwrap() {
            x_unit *= FieldElement::from(10u64);
        }

        let mut y_unit = FieldElement::from(10u64);
        for _ in 0..y_token_decimals.to_string().parse().unwrap() {
            y_unit *= FieldElement::from(10u64);
        }

        let x_balance = x.to_big_decimal(0) / x_unit.to_big_decimal(0);
        let y_balance = y.to_big_decimal(0) / y_unit.to_big_decimal(0);
        let x_price = y_balance.clone() / x_balance.clone();
        let y_price = x_balance.clone() / y_balance.clone();

        println!("x_balance: {}", x_balance);
        println!("y_balance: {}", y_balance);
        println!("k: {}", k.to_big_decimal(0));
        println!("x_price: {}", x_price);
        println!("y_price: {}", y_price);
        println!("- - - - - - - - - - - -");

        sleep(Duration::from_secs(polling_rate));
    }
}
