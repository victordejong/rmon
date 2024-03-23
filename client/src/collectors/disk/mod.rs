use crate::LiveMetrics;
use crate::HostFacts;

#[cfg(target_os = "linux")]
mod linux;
use linux::{get_disk_stats, get_host_facts};

pub fn collect(mut live_metrics_struct: LiveMetrics, mut disks: Vec<String>) -> (LiveMetrics, Option<Vec<String>>) {

    disks = get_disk_stats(&mut live_metrics_struct, disks);

    return (live_metrics_struct, Some(disks));
}

pub fn collect_host_facts(mut host_facts_struct: HostFacts) -> HostFacts {
    get_host_facts(&mut host_facts_struct);
    return host_facts_struct;
}
