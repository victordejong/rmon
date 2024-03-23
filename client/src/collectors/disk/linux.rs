use procfs::{DiskStat, DiskStats, Current};
use crate::LiveMetrics;
use crate::HostFacts;

pub fn get_disk_stats(live_metrics_struct: &mut LiveMetrics) {
    let system_disks: Vec<DiskStat> = match DiskStats::current() {
        Err(error) => panic!("Cannot get DiskStats ProcFS struct: {}", error),
        Ok(result) => result.0,
    };

    for system_disk in system_disks.iter() {
        for config_disk in live_metrics_struct.disks.iter_mut() {
            if system_disk.name == config_disk.name {
                config_disk.total_reads = system_disk.reads;
                config_disk.total_writes = system_disk.writes;
                config_disk.current_io = system_disk.in_progress;
            }
        }
    }

    return;
}

pub fn get_host_facts(host_facts_struct: &mut HostFacts) {
    let system_disks: Vec<DiskStat> = match DiskStats::current() {
        Err(error) => panic!("Cannot get DiskStats ProcFS struct: {}", error),
        Ok(result) => result.0,
    };

    for system_disk in system_disks {
        host_facts_struct.disks.push(String::from(system_disk.name));
    }

    return;
}