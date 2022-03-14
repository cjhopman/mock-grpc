use std::time::Instant;

use crate::protogen::protos::mock::mock_service_client::MockServiceClient;
use crate::protogen::protos::mock::MockRequest;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt()]
    count: u32,

    #[structopt()]
    ttl: u64,
}

mod protogen;

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let mut client = MockServiceClient::connect("tcp://127.0.0.1:50052")
        .await
        .unwrap();
    let started_at = Instant::now();
    let mut stream = client
        .mock(MockRequest {
            number_of_streams: opt.count,
            buffer_size: 0,
            ttl: opt.ttl,
        })
        .await
        .unwrap()
        .into_inner();
    while let Some(m) = stream.message().await.unwrap() {
        eprintln!(
            "{:?} after {:.4}s",
            m,
            Instant::now().duration_since(started_at).as_secs_f64()
        );
    }
}
