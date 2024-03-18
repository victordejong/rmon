use crate::LiveMetrics;
use crate::HostFacts;

#[cfg(target_os = "linux")]
mod linux;
use linux::{get_mem_data, get_host_facts};

pub fn collect(mut live_metrics_struct: LiveMetrics) -> LiveMetrics {
    get_mem_data(&mut live_metrics_struct);

    return live_metrics_struct;
}

pub fn collect_host_facts(mut host_facts_struct: HostFacts) -> HostFacts {
    get_host_facts(&mut host_facts_struct);
    return host_facts_struct;
}
