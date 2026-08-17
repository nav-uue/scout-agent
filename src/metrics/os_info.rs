use sysinfo::System;
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct OsInfo {
    #[serde(rename = "System name")]
    system_name: Vec<String>,
    #[serde(rename = "Kernel Version")]
    kernel_version: Vec<String>,
    #[serde(rename = "OS Version")]
    os_version: Vec<String>,
    #[serde(rename = "Host Name")]
    host_name: Vec<String>
}


pub fn collect() -> OsInfo {

    // Отримуємо значення, або "Unknown", якщо сталась помилка
    let system_name = System::name().unwrap_or(String::from("Unknown OS"));
    let kernel_version = System::kernel_version().unwrap_or(String::from("Unknown Kernel"));
    let os_version = System::os_version().unwrap_or(String::from("Unknown Version"));
    let host_name = System::host_name().unwrap_or(String::from("Unknown Host Name"));

    OsInfo {
        system_name: vec![system_name],
        kernel_version: vec![kernel_version],
        os_version: vec![os_version],
        host_name: vec![host_name]
    }

}