use crate::LiveMetrics;

#[cfg(target_os = "linux")]
mod linux;
use linux::get_disk_stats;

pub fn collect(mut live_metrics_struct: LiveMetrics, mut disks: Vec<String>) -> (LiveMetrics, Option<Vec<String>>) {

    disks = get_disk_stats(&mut live_metrics_struct, disks);

    return (live_metrics_struct, Some(disks));
}