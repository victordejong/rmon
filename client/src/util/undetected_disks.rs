use crate::LiveMetrics;
use crate::HostFacts;

pub fn warn_undetected_disks(live_metrics_struct: &LiveMetrics, host_facts_struct: &HostFacts) {

    let mut detected: bool = false;

    for conf_disk in live_metrics_struct.disks.iter() {
        for system_disk in host_facts_struct.disks.iter() {
            if &conf_disk.name == system_disk {
                detected = true;
                break;
            }
        }

        if detected {
            detected = false;
        } else {
            println!("WARNING: configured disk {} not detected!", conf_disk.name);
        }
    }
}
