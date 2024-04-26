use tonic::{transport::Server, Request, Response, Status};
use std::net::ToSocketAddrs;

use live_metrics_protobuf::live_metrics_greeter_server::{LiveMetricsGreeter, LiveMetricsGreeterServer};
use live_metrics_protobuf::{LiveMetricsMessage, LiveMetricsReply};

use host_facts_protobuf::host_facts_greeter_server::{HostFactsGreeter, HostFactsGreeterServer};
use host_facts_protobuf::{HostFactsMessage, HostFactsReply};

pub mod live_metrics_protobuf { tonic::include_proto!("livemetrics"); }

pub mod host_facts_protobuf { tonic::include_proto!("hostfacts"); }

mod printer;
mod config;

#[derive(Debug, Default)]
pub struct ServerLiveMetricsGreeter {}

#[derive(Debug, Default)]
pub struct ServerHostFactsGreeter {}

#[tonic::async_trait]
impl LiveMetricsGreeter for ServerLiveMetricsGreeter {
    async fn send_live_metrics(
        &self,
        request: Request<LiveMetricsMessage>,
    ) -> Result<Response<LiveMetricsReply>, Status> {
        //println!("Got a request: {:?}", request);
        printer::livemetrics_printer::print_received_live_metrics(&request);

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

    let config_struct: config::ConfigStruct = config::parse_config_sources();

//    println!("Started listening on {}", config_struct.listen_host.as_ref().unwrap());
    let addr = config_struct.listen_host.as_ref().unwrap().to_socket_addrs().unwrap().next().unwrap();

    let greeter = ServerLiveMetricsGreeter::default();
    let hostgreeter = ServerHostFactsGreeter::default();

    println!("Started listening on {}", config_struct.listen_host.as_ref().unwrap());

    Server::builder()
        .add_service(LiveMetricsGreeterServer::new(greeter))
        .add_service(HostFactsGreeterServer::new(hostgreeter))
        .serve(addr)
        .await?;

    Ok(())
}
