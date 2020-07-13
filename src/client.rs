use crate::protogen::protos::mock::mock_service_client::MockServiceClient;
use crate::protogen::protos::mock::MockRequest;

mod protogen;

#[tokio::main]
async fn main() {
  let mut client = MockServiceClient::connect("tcp://127.0.0.1:50052")
    .await
    .unwrap();
  let mut stream = client
    .mock(MockRequest {
      number_of_streams: 5,
      buffer_size: 0,
      ttl: 10,
    })
    .await
    .unwrap()
    .into_inner();
  while let Some(m) = stream.message().await.unwrap() {
    dbg!(m);
  }
}
