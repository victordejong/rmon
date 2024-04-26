use crate::HostFacts;
use crate::LiveMetrics;
use procfs::{Current, Meminfo};

pub fn get_mem_data(live_metrics_struct: &mut LiveMetrics) {
    let memstruct: Meminfo = match Meminfo::current() {
        Err(error) => panic!("Cannot get Meminfo ProcFS struct: {}", error),
        Ok(result) => result,
    };

    live_metrics_struct.mem.free = memstruct.mem_free;
    live_metrics_struct.mem.available = match memstruct.mem_available {
        None => panic!("Kernel does not support Meminfo::mem_available"),
        Some(result) => result,
    };
    live_metrics_struct.mem.cached = memstruct.cached;

    return;
}

pub fn get_host_facts(host_facts_struct: &mut HostFacts) {
    let memstruct: Meminfo = match Meminfo::current() {
        Err(error) => panic!("Cannot get Meminfo ProcFS struct: {}", error),
        Ok(result) => result,
    };

    host_facts_struct.mem.ram_total = memstruct.mem_total;
    host_facts_struct.mem.swap_total = memstruct.swap_total;

    return;
}
