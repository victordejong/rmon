pub mod host_facts_protobuf {
    tonic::include_proto!("hostfacts");
}

use host_facts_protobuf::host_facts_greeter_client::HostFactsGreeterClient;
use host_facts_protobuf::HostFactsMessage;
use host_facts_protobuf::host_facts_message::{HostCpuMessage,HostMemMessage,HostSystemMessage};
use tonic::Request;

use crate::data_structs::host_facts::HostFacts;

pub async fn send_host_facts_to_remote(remote_host: &String, host_facts: &HostFacts) {
    let mut client = HostFactsGreeterClient::connect(format!("http://{}", remote_host)).await
        .expect(&format!("ERROR: connection to server http://{} failed!", remote_host));

    let request: Request<HostFactsMessage> = tonic::Request::new(HostFactsMessage {
        cpu: Some(HostCpuMessage {
            cores: (host_facts.cpu.cores as u64),
            vendor_id: String::from(&host_facts.cpu.vendor_id),
            model_name: String::from(&host_facts.cpu.model_name),
        }),
        mem: Some(HostMemMessage {
            ram_total: host_facts.mem.ram_total,
            swap_total: host_facts.mem.swap_total,
        }),
        disks: host_facts.disks.clone(),
        system: Some(HostSystemMessage {
            hostname: String::from(&host_facts.system.hostname),
        })
    });

    client.send_host_facts(request).await.unwrap();
}
