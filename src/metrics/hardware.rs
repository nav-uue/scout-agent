use sysinfo::{Components, Disk, System};
use serde::Serialize;
use std::collections::HashMap;


#[derive(Debug, Serialize)]
pub struct HardwareInfo {
    #[serde(rename = "Total Memory")]
    total_memory: Vec<String>,
    #[serde(rename = "Used Memory")]
    used_memory: Vec<String>,
    #[serde(rename = "Total Swap")]
    total_swap: Vec<String>,
    #[serde(rename = "Used Swap")]
    used_swap: Vec<String>,
    cpus: HashMap<String, Vec<String>>
}


pub fn collect() -> HardwareInfo {

    let mut cpu_map = HashMap::new();

    // Initialize and refresh all system data
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let total_memory = format!("{}", sys.total_memory());
    let used_memory = format!("{}", sys.used_memory());
    let total_swap = format!("{}", sys.total_swap());
    let used_swap = format!("{}", sys.used_swap());

    for cpu in sys.cpus().iter() {
        
        let cpu_name = format!("Name: '{}'", cpu.name());
        let cpu_usage = format!("Usage: {:.2}%", cpu.cpu_usage());
        let cpu_frequency = format!("Freq: {} MHz", cpu.frequency());

        let metrics_vector = vec![cpu_usage, cpu_frequency];

        cpu_map.insert(cpu_name.clone().to_string(), metrics_vector);

    }

    HardwareInfo {
        total_memory: vec![total_memory],
        used_memory: vec![used_memory],
        total_swap: vec![total_swap],
        used_swap: vec![used_swap],
        cpus: cpu_map
    }

}