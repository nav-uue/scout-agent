use sysinfo::System;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct OsInfo {
    #[serde(rename = "System name")]
    system_name: String,
    #[serde(rename = "Kernel Version")]
    kernel_version: String,
    #[serde(rename = "OS Version")]
    os_version: String,
    #[serde(rename = "Host Name")]
    host_name: String
}


pub fn collect() -> OsInfo {

    // Retrieve value, or Unknown on error
    let system_name = System::name().unwrap_or(String::from("Unknown OS"));
    let kernel_version = System::kernel_version().unwrap_or(String::from("Unknown Kernel"));
    let os_version = System::os_version().unwrap_or(String::from("Unknown Version"));
    let host_name = System::host_name().unwrap_or(String::from("Unknown Host Name"));

    OsInfo {
        system_name: system_name,
        kernel_version: kernel_version,
        os_version: os_version,
        host_name: host_name
    }

}