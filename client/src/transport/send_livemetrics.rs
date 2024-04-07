pub mod live_metrics_protobuf {
    tonic::include_proto!("livemetrics");
}

use live_metrics_protobuf::live_metrics_greeter_client::LiveMetricsGreeterClient;
use live_metrics_protobuf::LiveMetricsMessage;
use live_metrics_protobuf::live_metrics_message::{LiveCpuMessage,LiveCpuLoad,LiveMemMessage,LiveDiskMessage};
use tonic::Request;

use crate::data_structs::live_metrics::LiveMetrics;

pub async fn send_live_metrics_to_remote(remote_host: &String, live_metrics: &LiveMetrics, hostname: &String) {
    let mut client = match LiveMetricsGreeterClient::connect(format!("http://{}", remote_host)).await {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR: connection to server http://{} failed: {}", remote_host, e);
            return;
        }
    };

    let mut output: LiveMetricsMessage = LiveMetricsMessage {
        cpu: Some(LiveCpuMessage {
            freq: live_metrics.cpu.freq,
            cpu_load: Some(LiveCpuLoad {
                load_1m: live_metrics.cpu.cpu_load.load_1m,
                load_5m: live_metrics.cpu.cpu_load.load_5m,
                load_15m: live_metrics.cpu.cpu_load.load_15m,
            })
        }),
        mem: Some(LiveMemMessage {
            free: live_metrics.mem.free,
            available: live_metrics.mem.available,
            cached: live_metrics.mem.cached,
        }),
        disks: vec![],
        hostname: String::from(hostname),
    };

    for disk in live_metrics.disks.iter() {
        output.disks.push(LiveDiskMessage {
            name: String::from(&disk.name),
            total_reads: disk.total_reads,
            total_writes: disk.total_writes,
            current_io: disk.current_io,
        });
    }

    let request: Request<LiveMetricsMessage> = tonic::Request::new(output);

    client.send_live_metrics(request).await.unwrap();
}
