use sysinfo::{Components, Disks, System};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;


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
    cpus: HashMap<String, Vec<String>>,
    storage: HashMap<String, Vec<String>>
}


pub fn collect() -> HardwareInfo {

    let mut cpu_map = HashMap::new();

    // Initialize and refresh all system data
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // RAM and SWAP info
    let total_memory = format!("{}", sys.total_memory() / (1024 * 1024));
    let used_memory = format!("{}", sys.used_memory() / (1024 * 1024));
    let total_swap = format!("{}", sys.total_swap() / (1024 * 1024));
    let used_swap = format!("{}", sys.used_swap() / (1024 * 1024));

    // CPU info
    for cpu in sys.cpus().iter() {
        
        let cpu_name = format!("{}", cpu.name());
        let cpu_usage = format!("Usage: {:.2}%", cpu.cpu_usage());
        let cpu_frequency = format!("Freq: {} MHz", cpu.frequency());

        let metrics_vector = vec![cpu_usage, cpu_frequency];

        cpu_map.insert(cpu_name, metrics_vector);

    }

    // Storage disks
    let mut disk_map = HashMap::new();

    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {

        let disk_path = Path::new(disk.name());
        let disk_name: String = disk_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| String::from("Unknown disk"));

        let disk_type = format!("Type: {}", disk.kind());
        let disk_fs = format!("Filesystem: {}", disk.file_system().to_string_lossy().into_owned());
        let disk_mp = format!("Mount point: {}", disk.mount_point().to_string_lossy().into_owned());

        let total_space = format!("Total space: {}", disk.total_space() / (1024 * 1024));
        let avalible_space = format!("Avalible space: {}", disk.available_space() / (1024 * 1024));

        let metrics_vector = vec![disk_type, disk_fs, disk_mp, total_space, avalible_space];

        disk_map.insert(disk_name, metrics_vector);

    }

    HardwareInfo {
        total_memory: vec![total_memory],
        used_memory: vec![used_memory],
        total_swap: vec![total_swap],
        used_swap: vec![used_swap],
        cpus: cpu_map,
        storage: disk_map
    }

}