pub struct LiveMetrics {
    pub cpu: Cpu,
    pub mem: Mem,
}

pub struct Cpu {
    pub amount: usize,
    pub freq: f32,
    pub cpu_load: CpuLoad,
}

pub struct CpuLoad {
    pub load_1m: f32,
    pub load_5m: f32,
    pub load_15m: f32,
}

pub struct Mem {
    pub free: u64,
    pub cached: u64,
}

pub fn init_live_metrics_struct() -> LiveMetrics {
    let live_metrics_struct = LiveMetrics {
        cpu: Cpu {
            amount: 0,
            freq: 0.,
            cpu_load: CpuLoad {
                load_1m: 0.,
                load_5m: 0.,
                load_15m: 0.,
            },            
        },
        mem: Mem {
            free: 0,
            cached: 0,
        }
    };

    return live_metrics_struct;
}