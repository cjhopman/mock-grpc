use crate::protogen::protos::mock::mock_service_server::{MockService, MockServiceServer};
use crate::protogen::protos::mock::{MockRequest, MockResponse};
use futures::channel::mpsc;
use futures::SinkExt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, time::Instant};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod protogen;

#[derive(Debug, Default)]
pub struct Mock {}

#[tonic::async_trait]
impl MockService for Mock {
    type MockStream = mpsc::Receiver<Result<MockResponse, Status>>;

    async fn mock(
        &self,
        request: Request<MockRequest>,
    ) -> Result<Response<Self::MockStream>, Status> {
        let req = request.into_inner();
        let (mut tx, rx) = mpsc::channel(req.buffer_size as usize);
        let started_at = Instant::now();

        tokio::spawn(async move {
            let mut count = 1;

            loop {
                if Instant::now().duration_since(started_at).as_secs() > req.ttl {
                    break;
                };

                while count <= req.number_of_streams {
                    println!("Sending stream {}...", count);
                    match tx
                        .send(Ok(MockResponse {
                            stream_id: count,
                            created_at: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        }))
                        .await
                    {
                        Ok(_) => {
                            println!(
                                "Sent stream {} after {:.4} secs",
                                count,
                                Instant::now().duration_since(started_at).as_secs_f64()
                            );
                        }
                        Err(_) => {}
                    };

                    count += 1;
                }
            }
        });

        Ok(Response::new(rx))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("50052"),
    };

    println!("{}", port);

    let address = format!("127.0.0.1:{}", port).parse()?;
    let mock = Mock::default();

    let mut server = Server::builder();

    println!("GO FOR LAUNCH");
    server
        .add_service(MockServiceServer::new(mock))
        .serve(address)
        .await?;

    Ok(())
}
