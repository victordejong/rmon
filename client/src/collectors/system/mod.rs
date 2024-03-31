use crate::HostFacts;

#[cfg(target_os = "linux")]
mod linux;
use linux::get_host_facts;

pub fn collect_host_facts(mut host_facts_struct: HostFacts) -> HostFacts {
    get_host_facts(&mut host_facts_struct);
    return host_facts_struct;
}
