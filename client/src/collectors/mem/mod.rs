use  crate::data_structs::LiveMetrics;

#[cfg(target_os = "linux")]
mod linux;
use linux::get_mem_data;

pub fn collect(mut live_metrics_struct: LiveMetrics) -> LiveMetrics {
    get_mem_data(&mut live_metrics_struct);

    return live_metrics_struct;
}
