use  crate::data_structs::LiveMetrics;

#[cfg(target_os = "linux")]
mod linux;
use linux::get_cpu_data;

pub fn collect_cpu_data(mut live_metrics_struct: LiveMetrics) -> LiveMetrics {

    get_cpu_data(&mut live_metrics_struct);

    return live_metrics_struct;
}
