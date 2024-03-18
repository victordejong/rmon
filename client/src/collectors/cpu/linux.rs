use procfs::{CpuInfo, Current, LoadAverage};
use crate::LiveMetrics;
use crate::HostFacts;

pub fn get_cpu_data(live_metrics_struct: &mut LiveMetrics) {

    let cpustruct: CpuInfo = match CpuInfo::current() {
        Err(error) => panic!("Cannot get CpuInfo ProcFS struct: {}", error),
        Ok(result) => result,
    };

    let cpu_cores: usize = CpuInfo::num_cores(&cpustruct);

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

pub fn get_system_load(live_metrics_struct: &mut LiveMetrics) {
    let loadstruct: LoadAverage = match LoadAverage::current() {
        Err(error) => panic!("Cannot get LoadAverage ProcFS struct: {}", error),
        Ok(result) => result,
    };

    live_metrics_struct.cpu.cpu_load.load_1m = loadstruct.one;
    live_metrics_struct.cpu.cpu_load.load_5m = loadstruct.five;
    live_metrics_struct.cpu.cpu_load.load_15m = loadstruct.fifteen;

    return;
}

pub fn get_host_facts(host_facts_struct: &mut HostFacts) {
    let cpustruct: CpuInfo = match CpuInfo::current() {
        Err(error) => panic!("Cannot get CpuInfo ProcFS struct: {}", error),
        Ok(result) => result,
    };

    host_facts_struct.cpu.cores = CpuInfo::num_cores(&cpustruct);
    host_facts_struct.cpu.vendor_id = match CpuInfo::vendor_id(&cpustruct, 0){
        None => panic!("No CPU 0, what does this even mean?"),
        Some(result) => result.to_string(),
    };
    host_facts_struct.cpu.model_name = match CpuInfo::model_name(&cpustruct, 0){
        None => panic!("No CPU 0, what does this even mean?"),
        Some(result) => result.to_string(),
    };

    return;
}
