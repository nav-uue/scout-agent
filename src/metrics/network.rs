use sysinfo::Networks;
use serde::Serialize;
use std::collections::HashMap;


#[derive(Debug, Serialize)]
pub struct NetworkInfo {
    interfaces: HashMap<String, Vec<String>>,
}


pub fn collect() -> NetworkInfo {

    let mut interfaces_map = HashMap::new();
    
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {

        let mac = format!("MAC: {}", data.mac_address());
        let rx = format!("RX: {} B", data.total_received());
        let tx = format!("TX: {} B", data.total_transmitted());
        
        let metrics_vector = vec![mac, rx, tx];
        
        interfaces_map.insert(interface_name.clone(), metrics_vector);

    }

    NetworkInfo {
        interfaces: interfaces_map,
    }

}