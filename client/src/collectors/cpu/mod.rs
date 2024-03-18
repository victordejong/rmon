use crate::LiveMetrics;
use crate::HostFacts;

#[cfg(target_os = "linux")]
mod linux;
use linux::{get_cpu_data, get_system_load, get_host_facts};

pub fn collect(mut live_metrics_struct: LiveMetrics) -> LiveMetrics {

    get_cpu_data(&mut live_metrics_struct);
    get_system_load(&mut live_metrics_struct);

    return live_metrics_struct;
}

pub fn collect_host_facts(mut host_facts_struct: HostFacts) -> HostFacts {
    get_host_facts(&mut host_facts_struct);
    return host_facts_struct;
}
