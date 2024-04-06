use tonic::{transport::Server, Request, Response, Status};
use std::net::SocketAddr;

use live_metrics_protobuf::greeter_server::{Greeter, GreeterServer};
use live_metrics_protobuf::{LiveMetricsMessage, LiveMetricsReply};
use live_metrics_protobuf::live_metrics_message::{CpuMessage,MemMessage,SystemMessage};

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
        //println!("Got a request: {:?}", request);
        print_received_host_facts(request);

        let reply = live_metrics_protobuf::LiveMetricsReply {
            success: true,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

fn print_received_host_facts(request: Request<LiveMetricsMessage>) {

    let request_message: &LiveMetricsMessage = request.get_ref();
    let remote_addr: SocketAddr = request.remote_addr().unwrap();

    let system_message: &SystemMessage = match &request_message.system {
        None => {
            println!("ERROR: Malformed message received from {}!", remote_addr);
            return;
        }
        Some(system) => system
    };

    let cpu_message: &CpuMessage = match &request_message.cpu {
        None => {
            println!("ERROR: Malformed CpuMessage received from {} ({})!", system_message.hostname, remote_addr);
            return;
        }
        Some(cpu) => cpu
    };

    let mem_message: &MemMessage = match &request_message.mem {
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
