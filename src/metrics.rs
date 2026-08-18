use serde::Serialize;


// include files from the metrics/ folder
mod os_info;
mod hardware;
mod network;
mod sys_status;


#[derive(Debug, Serialize)]
pub struct MetricsReport {
    pub host_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_info: Option<os_info::OsInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<hardware::HardwareInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<network::NetworkInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_status: Option<sys_status::SysStatusInfo>,
}


pub fn build_report(host_id: String) -> MetricsReport {
    let os_info_data = os_info::collect();
    let hardware_data = hardware::collect();
    let network_data = network::collect();
    let sys_status_data = sys_status::collect();

    MetricsReport {
        host_id,
        os_info: Some(os_info_data),
        hardware: Some(hardware_data),
        network: Some(network_data),
        sys_status: Some(sys_status_data)
    }
}