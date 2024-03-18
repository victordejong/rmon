use procfs::{Meminfo, Current};
use  crate::data_structs::LiveMetrics;

pub fn get_mem_data(live_metrics_struct: &mut LiveMetrics) {
    let memstruct: Meminfo = match Meminfo::current() {
        Err(error) => panic!("Cannot get Meminfo ProcFS struct: {}", error),
        Ok(result) => result,
    };

    live_metrics_struct.mem.free = memstruct.mem_free;
    live_metrics_struct.mem.cached = memstruct.cached;

    return;
}
