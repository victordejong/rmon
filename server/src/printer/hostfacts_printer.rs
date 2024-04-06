use tonic::Request;
use std::net::SocketAddr;

use host_facts_protobuf::HostFactsMessage;
use host_facts_protobuf::host_facts_message::{HostCpuMessage,HostMemMessage,HostSystemMessage};
use crate::host_facts_protobuf;

pub fn print_received_host_facts(request: &Request<HostFactsMessage>) {

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
