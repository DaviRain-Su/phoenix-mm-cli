use ellipsis_client::EllipsisClient;
use phoenix_sdk::sdk_client::SDKClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::time::Duration;

#[derive(Debug, StructOpt)]
pub struct FetchMarketEvent {}

impl FetchMarketEvent {
    pub async fn run(&self) -> anyhow::Result<()> {
        let payer = Keypair::new();
        let client = EllipsisClient::from_rpc(
            RpcClient::new_with_commitment(
                "https://api.mainnet-beta.solana.com".to_string(),
                CommitmentConfig::confirmed(),
            ),
            &payer,
        )?;

        let sdk_client = Arc::new(SDKClient::new_from_ellipsis_client(client).await?);
        let sol_market = Pubkey::from_str("4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg")?;

        let mut until = None;
        loop {
            let config = match until {
                None => GetConfirmedSignaturesForAddress2Config {
                    before: None,
                    until: None,
                    limit: Some(1),
                    commitment: Some(CommitmentConfig::confirmed()),
                },
                Some(until) => GetConfirmedSignaturesForAddress2Config {
                    before: None,
                    until: Some(until),
                    limit: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            };

            let signatures = sdk_client
                .client
                .get_signatures_for_address_with_config(&sol_market, config)
                .await
                .unwrap_or_default()
                .iter()
                .map(|tx| Signature::from_str(&tx.signature).unwrap())
                .rev()
                .collect::<Vec<_>>();

            if !signatures.is_empty() {
                until = Some(signatures[0]);
            }

            let mut handles = vec![];

            for signature in signatures {
                let sdk = sdk_client.clone();
                let handle =
                    tokio::spawn(
                        async move { sdk.parse_events_from_transaction(&signature).await },
                    );
                handles.push(handle);
            }

            for handle in handles {
                let events = handle.await?;

                if let Some(events) = events {
                    events.iter().for_each(|e| {
                        // Here we only print the event, but in practice, you can do
                        // a lot more
                        println!("{:#?}", e);
                    });
                }
            }

            // Note: this is a basic polling loop, if there are >1000 signatures in 200ms
            // events will get dropped
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    }
}
