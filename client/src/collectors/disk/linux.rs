use procfs::{DiskStat, DiskStats, Current};
use crate::LiveMetrics;

pub fn get_disk_stats(live_metrics_struct: &mut LiveMetrics, disks: Vec<String>) -> Vec<String> {
    let system_disks: Vec<DiskStat> = match DiskStats::current() {
        Err(error) => panic!("Cannot get DiskStats ProcFS struct: {}", error),
        Ok(result) => result.0,
    };

    // TODO: What the fuck?
    for system_disk in system_disks.iter() {
        for disk in disks.iter() {
            if &system_disk.name == disk {
                for metric_disk in live_metrics_struct.disks.iter_mut() {
                    if &metric_disk.name == disk {
                        metric_disk.total_reads = system_disk.reads;
                        metric_disk.total_writes = system_disk.writes;
                        metric_disk.current_io = system_disk.in_progress;
                    }
                }
            }
        }
    }

    return disks;
}