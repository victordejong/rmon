use procfs::{CpuInfo, Current};
use  crate::data_structs;

pub fn get_cpu_data(live_metrics_struct: &mut data_structs::LiveMetrics) {

    let cpustruct = match procfs::CpuInfo::current() {
        Err(error) => panic!("Cannot get CpuInfo struct: {}", error),
        Ok(result) => result,
    };
    
    let cpu_cores: usize = procfs::CpuInfo::num_cores(&cpustruct);
    live_metrics_struct.cpu.amount = cpu_cores;

    let mut total: f32 = 0.;
    for n in 0..cpu_cores {

        let core_speed: String = match CpuInfo::get_field(&cpustruct, n, "cpu MHz") {
            None => panic!("No current frequency value for CPU {}", n),
            Some(result) => result.to_string(),
        };

        total += match core_speed.parse::<f32>() {
            Err(error) => panic!("CPU speed parsing to f32 failed with: {}", error),
            Ok(result) => result,
        };
    }

    live_metrics_struct.cpu.freq = total / cpu_cores as f32;

    return;
}
