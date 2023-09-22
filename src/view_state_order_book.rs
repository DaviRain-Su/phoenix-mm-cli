use ellipsis_client::EllipsisClient;
use phoenix_sdk::sdk_client::SDKClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use structopt::StructOpt;

use crate::utils::get_payer_keypair;

#[derive(Debug, StructOpt)]
pub struct ViewStateOrderBook {}

impl ViewStateOrderBook {
    pub async fn run(&self) -> anyhow::Result<()> {
        let payer = get_payer_keypair();

        let client = EllipsisClient::from_rpc(
            RpcClient::new_with_commitment(
                "https://api.mainnet-beta.solana.com".to_string(),
                CommitmentConfig::confirmed(),
            ),
            &payer,
        )?;

        let sdk_client = SDKClient::new_from_ellipsis_client(client).await?;
        let sol_market = Pubkey::from_str("4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg")?;
        let orderbook = sdk_client.get_market_orderbook(&sol_market).await?;
        orderbook.print_ladder(5, 4);

        Ok(())
    }
}
