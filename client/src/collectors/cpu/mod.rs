use crate::LiveMetrics;

#[cfg(target_os = "linux")]
mod linux;
use linux::{get_cpu_data, get_system_load};

pub fn collect(mut live_metrics_struct: LiveMetrics) -> LiveMetrics {

    get_cpu_data(&mut live_metrics_struct);
    get_system_load(&mut live_metrics_struct);

    return live_metrics_struct;
}
