use tonic::{transport::Server, Request, Response, Status};

use live_metrics_protobuf::greeter_server::{Greeter, GreeterServer};
use live_metrics_protobuf::{LiveMetricsMessage, LiveMetricsReply};

pub mod live_metrics_protobuf {
    tonic::include_proto!("livemetrics");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn send_live_metrics(
        &self,
        request: Request<LiveMetricsMessage>,
    ) -> Result<Response<LiveMetricsReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = live_metrics_protobuf::LiveMetricsReply {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
