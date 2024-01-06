use starknet::{
    core::types::{BlockId, BlockTag, FieldElement, FunctionCall},
    macros::{felt, selector},
    providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider},
};
use std::{thread::sleep, time::Duration};
use url::Url;

struct StarkNetService {
    rpc_client: JsonRpcClient<HttpTransport>,
    swap_pool: FieldElement,
    polling_rate: u64,
}

pub fn to_unit(decimals: FieldElement) -> FieldElement {
    let mut unit = FieldElement::from(10u64);
    for _ in 0..decimals.to_string().parse().unwrap() {
        unit *= FieldElement::from(10u64);
    }
    unit
}

impl StarkNetService {
    pub fn new(swap_pool: FieldElement, endpoint: &str, polling_rate: u64) -> Self {
        let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(endpoint).unwrap()));

        StarkNetService {
            rpc_client,
            swap_pool,
            polling_rate,
        }
    }

    async fn fetch_token_decimals(&self, token_address: FieldElement) -> FieldElement {
        self.rpc_client
            .call(
                FunctionCall {
                    contract_address: token_address,
                    entry_point_selector: selector!("decimals"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap()[0]
    }

    async fn fetch_tokens(&self) -> (FieldElement, FieldElement) {
        let x_token = self
            .rpc_client
            .call(
                FunctionCall {
                    contract_address: self.swap_pool,
                    entry_point_selector: selector!("token0"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap()[0];

        let y_token = self
            .rpc_client
            .call(
                FunctionCall {
                    contract_address: self.swap_pool,
                    entry_point_selector: selector!("token1"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap()[0];
        (x_token, y_token)
    }

    pub async fn fetch_reserves(&self) -> Vec<FieldElement> {
        let reserves = self
            .rpc_client
            .call(
                FunctionCall {
                    contract_address: self.swap_pool,
                    entry_point_selector: selector!("get_reserves"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap();

        reserves
    }

    pub async fn fetch_k(&self) -> FieldElement {
        let klast = self
            .rpc_client
            .call(
                FunctionCall {
                    contract_address: self.swap_pool,
                    entry_point_selector: selector!("klast"),
                    calldata: vec![],
                },
                BlockId::Tag(BlockTag::Latest),
            )
            .await
            .unwrap();

        klast[0] + klast[1]
    }

    pub async fn start(&self) {
        loop {
            let (x_token, y_token) = self.fetch_tokens().await;
            let x_token_decimals = self.fetch_token_decimals(x_token).await;
            let y_token_decimals = self.fetch_token_decimals(y_token).await;

            let reserves = self.fetch_reserves().await;

            let x = reserves[0] + reserves[1];
            let y = reserves[2] + reserves[3];

            let k = self.fetch_k().await;

            let x_unit = to_unit(x_token_decimals);
            let y_unit = to_unit(y_token_decimals);

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

            sleep(Duration::from_secs(self.polling_rate));
        }
    }
}

#[tokio::main]
async fn main() {
    // Set the polling rate
    let polling_rate = 1;

    // ETH-USDC pool
    let swap_pool = felt!("0x04d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a");

    let starknet_service = StarkNetService::new(
        swap_pool,
        "https://starknet-mainnet.public.blastapi.io",
        polling_rate,
    );

    starknet_service.start().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_service() -> StarkNetService {
        StarkNetService::new(
            felt!("0x04d0390b777b424e43839cd1e744799f3de6c176c7e32c1812a41dbd9c19db6a"),
            "https://starknet-mainnet.public.blastapi.io",
            10,
        )
    }

    #[tokio::test]
    async fn test_fetch_token_decimals() {
        let service = setup_service();
        let decimals = service
            .fetch_token_decimals(felt!(
                "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
            ))
            .await;
        assert_eq!(decimals, FieldElement::from(18u64));
    }

    #[tokio::test]
    async fn test_fetch_tokens() {
        let service = setup_service();
        let (x_token, y_token) = service.fetch_tokens().await;
        assert_eq!(
            x_token,
            felt!("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7")
        );
        assert_eq!(
            y_token,
            felt!("0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8")
        );
    }

    #[tokio::test]
    async fn test_fetch_reserves() {
        let service = setup_service();
        let reserves = service.fetch_reserves().await;
        assert_eq!(reserves.len(), 5);
    }
}
