use sysinfo::System;
use std::collections::HashMap;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct SysStatusInfo {
    top_process: HashMap<String, Vec<String>>
}


pub fn collect() -> SysStatusInfo {

    let mut process_map = HashMap::new();

    // Initialize and refresh all system data
    let mut sys = System::new_all();
    sys.refresh_all();

    for (pid, process) in sys.processes().iter().take(5) {

        let name = process.name().to_string_lossy().into_owned();
        let pid = format!("PID: {:5}", pid);
        let cpu = format!("CPU: {:5.2}%", process.cpu_usage());
        let mem = format!("Memory: {} KB", process.memory());
        let stat = format!("Status: {}", process.status());

        let process_info = vec![pid, cpu, mem, stat];

        process_map.insert(name, process_info);

    }

    SysStatusInfo {
        top_process: process_map
    }

}