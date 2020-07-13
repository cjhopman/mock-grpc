use crate::protogen::protos::mock::mock_service_server::{MockService, MockServiceServer};
use crate::protogen::protos::mock::{MockRequest, MockResponse};
use futures::channel::mpsc;
use futures::SinkExt;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
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

        tokio::spawn(async move {
            let started_at = SystemTime::now();
            let mut count = 1;

            loop {
                if SystemTime::now()
                    .duration_since(started_at)
                    .unwrap()
                    .as_secs()
                    > req.ttl
                {
                    break;
                };

                while count <= req.number_of_streams {
                    let now = SystemTime::now();

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
                                "Sent stream {} in {} nanos",
                                count,
                                SystemTime::now().duration_since(now).unwrap().as_nanos()
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
