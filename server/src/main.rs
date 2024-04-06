use tonic::{transport::Server, Request, Response, Status};
use std::net::SocketAddr;

use live_metrics_protobuf::greeter_server::{Greeter, GreeterServer};
use live_metrics_protobuf::{LiveMetricsMessage, LiveMetricsReply};
use live_metrics_protobuf::live_metrics_message::{CpuMessage,MemMessage,SystemMessage};

use host_facts_protobuf::host_facts_greeter_server::{HostFactsGreeter, HostFactsGreeterServer};
use host_facts_protobuf::{HostFactsMessage, HostFactsReply};
use host_facts_protobuf::host_facts_message::{HostCpuMessage,HostMemMessage,HostSystemMessage};

pub mod live_metrics_protobuf {
    tonic::include_proto!("livemetrics");
}

pub mod host_facts_protobuf {
    tonic::include_proto!("hostfacts");
}

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
        print_received_host_facts(request);

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

fn print_received_host_facts(request: Request<HostFactsMessage>) {

    let request_message: &HostFactsMessage = request.get_ref();
    let remote_addr: SocketAddr = request.remote_addr().unwrap();

    let system_message: &HostSystemMessage = match &request_message.system {
        None => {
            println!("ERROR: Malformed message received from {}!", remote_addr);
            return;
        }
        Some(system) => system
    };

    let cpu_message: &HostCpuMessage = match &request_message.cpu {
        None => {
            println!("ERROR: Malformed CpuMessage received from {} ({})!", system_message.hostname, remote_addr);
            return;
        }
        Some(cpu) => cpu
    };

    let mem_message: &HostMemMessage = match &request_message.mem {
        None => {
            println!("ERROR: Malformed MemMessage received from {} ({})!", system_message.hostname, remote_addr);
            return;
        }
        Some(mem) => mem
    };

    println!("Received new/updated host facts for {} ({}): CPU: {} cores, {} | MEM: {:.2} GB | Disks: {:?}", system_message.hostname,
                remote_addr, cpu_message.cores, cpu_message.model_name, mem_message.ram_total as f32 / 1e9,
                &request_message.disks);

}
