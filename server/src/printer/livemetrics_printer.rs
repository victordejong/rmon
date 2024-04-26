use std::net::SocketAddr;
use tonic::Request;

use crate::live_metrics_protobuf;
use live_metrics_protobuf::live_metrics_message::{
    LiveCpuMessage, LiveDiskMessage, LiveMemMessage,
};
use live_metrics_protobuf::LiveMetricsMessage;

pub fn print_received_live_metrics(request: &Request<LiveMetricsMessage>) {
    let request_message: &LiveMetricsMessage = request.get_ref();
    let remote_addr: SocketAddr = request.remote_addr().unwrap();

    let cpu_message: &LiveCpuMessage = match &request_message.cpu {
        None => {
            println!("ERROR: Malformed message received from {}!", remote_addr);
            return;
        }
        Some(cpu) => cpu,
    };

    let mem_message: &LiveMemMessage = match &request_message.mem {
        None => {
            println!(
                "ERROR: Malformed MemMessage received from {} ({})!",
                request_message.hostname, remote_addr
            );
            return;
        }
        Some(mem) => mem,
    };

    let disk_message: &Vec<LiveDiskMessage> = &request_message.disks;

    print!(
        "Metrics update received for {} ({}):",
        request_message.hostname, remote_addr
    );
    print!(
        "CPU: freq avg {:.2} GHz, load {} {} {}; ",
        cpu_message.freq / 1024.,
        &cpu_message.cpu_load.as_ref().unwrap().load_1m,
        &cpu_message.cpu_load.as_ref().unwrap().load_5m,
        &cpu_message.cpu_load.as_ref().unwrap().load_15m
    );

    print!(
        "MEM: free {:.2} Gb, available {:.2} Gb, cached {:.2} Mb; ",
        mem_message.free as f32 / 1e9,
        mem_message.available as f32 / 1e9,
        mem_message.cached as f32 / 1e6
    );

    // Print disk details
    print!("DISK:");
    for disk in disk_message.iter() {
        print!(
            " {}: total writes {}, total reads {}, current IO: {} |",
            disk.name, disk.total_writes, disk.total_reads, disk.current_io
        );
    }

    print!("; ");

    println!("");
    return;
}
