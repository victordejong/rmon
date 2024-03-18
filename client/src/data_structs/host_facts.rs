/**
Desciption: This struct holds all inventory facts for a host. The metrics this struct are collected once during startup
            and are considered static for the duration the program is expected to compute.
*/

use crate::collectors::{cpu, mem};

pub struct HostFacts {
    pub cpu: Cpu,
    pub mem: Mem,
}

pub struct Cpu {
    pub cores: usize,
    pub vendor_id: String,
    pub model_name: String,
}

pub struct Mem {
    pub ram_total: u64,
    pub swap_total: u64,
}

pub fn init_host_facts_struct() -> HostFacts {
    let mut host_facts = HostFacts {
        cpu: Cpu {
            cores: 0,
            vendor_id: "".to_string(),
            model_name: "".to_string(),
        },
        mem: Mem {
            ram_total: 0,
            swap_total: 0,
        },
    };

    host_facts = populate_host_facts_struct(host_facts);

    return host_facts;
}

fn populate_host_facts_struct(mut host_facts_struct: HostFacts) -> HostFacts {
    host_facts_struct = cpu::collect_host_facts(host_facts_struct);
    host_facts_struct = mem::collect_host_facts(host_facts_struct);
    return host_facts_struct;
}
