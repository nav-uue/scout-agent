use sysinfo::Networks;
use serde::Serialize;
use std::collections::HashMap;


#[derive(Debug, Serialize)]
pub struct NetworkInfo {
    interfaces: HashMap<String, Vec<String>>,
}


pub fn collect() -> NetworkInfo {

    let mut interfaces_map = HashMap::new();
    
    // Ваш код збору даних
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        // Формуємо красиві рядки для кожної метрики
        let mac = format!("MAC: {}", data.mac_address());
        let rx = format!("RX: {} B", data.total_received());
        let tx = format!("TX: {} B", data.total_transmitted());
        
        // Кладемо всі три метрики у вектор для цього інтерфейсу
        let metrics_vector = vec![mac, rx, tx];
        
        // Зберігаємо в мапу під іменем інтерфейсу (наприклад, "eth0")
        interfaces_map.insert(interface_name.clone(), metrics_vector);
    }

    // Повертаємо нашу структуру
    NetworkInfo {
        interfaces: interfaces_map,
    }

}