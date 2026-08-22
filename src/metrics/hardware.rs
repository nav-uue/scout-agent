use sysinfo::{Disks, System};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::fs;


#[derive(Debug, Serialize)]
pub struct HardwareInfo {
    system_info: HashMap<String, Vec<String>>,
    memory: HashMap<String, Vec<String>>,
    cpus: HashMap<String, Vec<String>>,
    storage: HashMap<String, Vec<String>>
}


fn metadata_parser(name: String) -> Vec<String> {

    let dmi = fs::read_dir("/sys/class/dmi/id").unwrap();

    let mut metrics_vector: Vec<String> = Vec::new();

    for file in dmi.flatten() {

        let file_name_str = file.file_name().to_string_lossy().into_owned();

        if file_name_str.contains(&name) {

            let file_name = match file_name_str.split('_').last() {
                Some(n) => n,
                None => "Unknown"
            };

            let bios = fs::read_to_string(file.path()).unwrap_or_else(|_| format!("Unknown {}", &name));
            metrics_vector.push(format!("{}: {}", file_name, bios.trim_end()));

        }

    }

    metrics_vector

}


pub fn collect() -> HardwareInfo {

    // add metadata from /sys/class/dmi/id
    let mut system_info_map = HashMap::new();
    
    system_info_map.insert("bios".to_string(), metadata_parser("bios".to_string()));
    system_info_map.insert("board".to_string(), metadata_parser("board".to_string()));
    system_info_map.insert("chassis".to_string(), metadata_parser("chassis".to_string()));
    system_info_map.insert("product".to_string(), metadata_parser("product".to_string()));

    // Initialize and refresh all system data
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // RAM and SWAP info
    let mut memory_map = HashMap::new();

    let total_memory = format!("Total RAM: {}", sys.total_memory() / (1024 * 1024));
    let used_memory = format!("Used RAM: {}", sys.used_memory() / (1024 * 1024));
    let total_swap = format!("Total SWAP{}", sys.total_swap() / (1024 * 1024));
    let used_swap = format!("Used SWAP{}", sys.used_swap() / (1024 * 1024));

    memory_map.insert("RAM".to_string(), vec![total_memory, used_memory]);
    memory_map.insert("SWAP".to_string(), vec![total_swap, used_swap]);

    // CPU info
    let mut cpu_map = HashMap::new();

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
        system_info: system_info_map,
        memory: memory_map,
        cpus: cpu_map,
        storage: disk_map
    }

}