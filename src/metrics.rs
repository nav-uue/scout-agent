use sysinfo::{
    CpuRefreshKind, 
    MemoryRefreshKind, 
    RefreshKind, 
    System
};

pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub ram_usage_percent: f32,
}

pub struct MetricsCollector {
    sys: System,
}

impl MetricsCollector {

    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );
        sys.refresh_cpu_all();
        Self { sys }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        // refreshing data before reading
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        // Computing average CPU utilization
        let cpus = self.sys.cpus();
        let total_cpu: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        let cpu_usage = total_cpu / cpus.len() as f32;

        // Computing used RAM percentage
        let total_mem = self.sys.total_memory() as f32;
        let used_mem = self.sys.used_memory() as f32;
        let ram_usage_percent = (used_mem / total_mem) * 100.0;

        SystemMetrics {
            cpu_usage,
            ram_usage_percent,
        }
    }

}