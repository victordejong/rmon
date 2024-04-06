use tonic::{transport::Server, Request, Response, Status};

use live_metrics_protobuf::greeter_server::{Greeter, GreeterServer};
use live_metrics_protobuf::{LiveMetricsMessage, LiveMetricsReply};

use host_facts_protobuf::host_facts_greeter_server::{HostFactsGreeter, HostFactsGreeterServer};
use host_facts_protobuf::{HostFactsMessage, HostFactsReply};

pub mod live_metrics_protobuf {
    tonic::include_proto!("livemetrics");
}

pub mod host_facts_protobuf {
    tonic::include_proto!("hostfacts");
}

mod printer;

#[derive(Debug, Default)]
pub struct LiveMetricsGreeter {}

#[derive(Debug, Default)]
pub struct ServerHostFactsGreeter {}

#[tonic::async_trait]
impl Greeter for LiveMetricsGreeter {
    async fn send_live_metrics(
        &self,
        request: Request<LiveMetricsMessage>,
    ) -> Result<Response<LiveMetricsReply>, Status> {
        //println!("Got a request: {:?}", request);
        //print_received_host_facts(request);

        let reply = live_metrics_protobuf::LiveMetricsReply {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl HostFactsGreeter for ServerHostFactsGreeter {
    async fn send_host_facts(
        &self,
        request: Request<HostFactsMessage>,
    ) -> Result<Response<HostFactsReply>, Status> {
        //println!("Got a request: {:?}", request);
        printer::hostfacts_printer::print_received_host_facts(&request);

        let reply = host_facts_protobuf::HostFactsReply {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = LiveMetricsGreeter::default();
    let hostgreeter = ServerHostFactsGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(HostFactsGreeterServer::new(hostgreeter))
        .serve(addr)
        .await?;

    Ok(())
}
