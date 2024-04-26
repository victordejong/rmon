use crate::HostFacts;
use crate::LiveMetrics;

#[cfg(target_os = "linux")]
mod linux;
use linux::{get_cpu_data, get_host_facts, get_system_load};

pub fn collect(mut live_metrics_struct: LiveMetrics) -> LiveMetrics {
    get_cpu_data(&mut live_metrics_struct);
    get_system_load(&mut live_metrics_struct);

    return live_metrics_struct;
}

pub fn collect_host_facts(mut host_facts_struct: HostFacts) -> HostFacts {
    get_host_facts(&mut host_facts_struct);
    return host_facts_struct;
}
