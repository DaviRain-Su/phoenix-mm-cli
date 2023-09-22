pub mod grpc;
pub mod place_order;
pub mod sample;
pub mod sample_market_maker;

use grpc::Grpc;
use place_order::PlaceOrder;
use sample::Sample;
use sample_market_maker::SampleMarketMaker;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "grpc")]
    Grpc(Grpc),
    #[structopt(name = "place-order")]
    PlaceOrder(PlaceOrder),
    #[structopt(name = "sample")]
    Sample(Sample),
    #[structopt(name = "sample-market-maker")]
    SampleMarketMaker(SampleMarketMaker),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "phoneix-mm-cli")]
pub struct PhoneixMMCli {
    #[structopt(subcommand)]
    pub command: Command,
}

impl PhoneixMMCli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Grpc(grpc) => grpc.run().await,
            Command::PlaceOrder(place_order) => {
                place_order.run().await;
                Ok(())
            }
            Command::Sample(sample) => sample.run().await,
            Command::SampleMarketMaker(sample_market_maker) => sample_market_maker.run().await,
        }
    }
}

#[tokio::main]
async fn main() {
    let opt = PhoneixMMCli::from_args();
    let _ = opt.run().await;
}