use  crate::data_structs;

#[cfg(target_os = "linux")]
mod linux;
use linux::get_cpu_data;

pub fn collect_cpu_data(mut live_metrics_struct: data_structs::LiveMetrics) -> data_structs::LiveMetrics {

    get_cpu_data(&mut live_metrics_struct);

    return live_metrics_struct;
}
