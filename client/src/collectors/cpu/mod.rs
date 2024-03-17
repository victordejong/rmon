use cpu_freq::CpuFreqs;

use  crate::data_structs;

pub fn collect_cpu_data(live_metrics_struct: &mut data_structs::LiveMetrics) {
    let cpus: Vec<CpuFreqs> = cpu_freq::get();

    let freqs = get_individual_cpu_metrics(cpus);

    live_metrics_struct.cpu.freq = freqs.0;
    live_metrics_struct.cpu.amount = freqs.1;

    return;
}

fn get_individual_cpu_metrics(cpus: Vec<CpuFreqs>) -> (f32, usize) {

    let mut count: f32 = 0.;

    for (i, e) in cpus.iter().enumerate() {
        let cpu_freq_temp = match e.cur {
            None => panic!("No current frequency value for CPU {}", i),
            Some(element) => element,
        };

        count += cpu_freq_temp;
    }

    let amount = cpus.len();

    let avg = count / amount as f32;

    return (avg, amount);
}