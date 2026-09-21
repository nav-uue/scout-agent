use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


// include files from the metrics/ folder
mod os_info;
mod hardware;
mod network;
mod sys_status;


#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsReport {
    #[serde(flatten)]
    pub identifier: UniqueIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_info: Option<os_info::OsInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<hardware::HardwareInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<network::NetworkInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_status: Option<sys_status::SysStatusInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniqueIdentifier {
    pub host_id: String,
    pub datatime: DateTime<Local>,
}

pub fn unique_identifier() -> UniqueIdentifier {

    let identifier = UniqueIdentifier {
        host_id: host_id(),
        datatime: Local::now()
    };

    identifier

}

fn host_id() -> String {

    // // Get system info (vendor, serial, product name, UUID) from DMI files
    let path = String::from("/sys/class/dmi/id");

    let board_vendor = fs::read_to_string(format!("{path}/board_vendor")).unwrap_or_else(|_| format!("board_vendor"));
    let product_serial = fs::read_to_string(format!("{path}/product_serial")).unwrap_or_else(|_| format!("product_serial"));
    let product_name = fs::read_to_string(format!("{path}/product_name")).unwrap_or_else(|_| format!("product_name"));
    let product_uuid = fs::read_to_string(format!("{path}/product_uuid")).unwrap_or_else(|_| format!("product_uuid"));

    // Get the number of physical cores
    let mut sys = sysinfo::System::new();
    sys.refresh_cpu_list(sysinfo::CpuRefreshKind::nothing());
    let phisical_cups = sysinfo::System::physical_core_count().unwrap_or(0);

    // Get MAC address (physical devices only)
    let net_dir = std::path::Path::new("/sys/class/net");

    let mac_identify = if let Ok(entries) = fs::read_dir(net_dir) {

        let mut interfaces: Vec<String> = Vec::new();

        for entry in entries.filter_map(|e| e.ok()) {
            let iface_name = entry.file_name().to_string_lossy().into_owned();

            // Ignore loopback interface
            if iface_name == "lo" {
                continue;
            }

            // Ignore virtual interfaces (docker)
            let device_path = entry.path().join("device");
            if !device_path.exists() {
                continue; 
            }

            // Read the file containing the MAC address
            let address_path = entry.path().join("address");
            if let Ok(mac) = fs::read_to_string(address_path) {

                let mac_trimmed = mac.trim();
                
                if mac_trimmed != "00:00:00:00:00:00" && !mac_trimmed.is_empty() {
                    interfaces.push(mac_trimmed.to_string());
                }
                
            }
        }

        interfaces.sort();
        interfaces.join(" ")

    } else { String::from("Empty") };

    let params = format!(
        "{} | {} | {} | {} | {} | {}",
        board_vendor.trim().to_string(),
        product_serial.trim().to_string(),
        product_name.trim().to_string(),
        product_uuid.trim().to_string(),
        phisical_cups,
        mac_identify
    );

    let mut hasher = DefaultHasher::new();
    params.hash(&mut hasher);
    let hash_num = hasher.finish();
    let hash_str = format!("{:016x}", hash_num);

    let chuncks: Vec<&str> = hash_str
        .as_bytes()
        .chunks(4)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect();

    let id =chuncks.join("-");
    id // return host_id

}

pub fn build_report() -> MetricsReport {
    let os_info_data = os_info::collect();
    let hardware_data = hardware::collect();
    let network_data = network::collect();
    let sys_status_data = sys_status::collect();

    MetricsReport {
        identifier: unique_identifier(),
        os_info: Some(os_info_data),
        hardware: Some(hardware_data),
        network: Some(network_data),
        sys_status: Some(sys_status_data)
    }
}