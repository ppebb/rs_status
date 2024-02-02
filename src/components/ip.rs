use if_addrs::{get_if_addrs, IfAddr};

fn ip(interface: &str, interface_type: u8) -> Result<String, String> {
    let ifaddr = match get_if_addrs() {
        Ok(if_addr) => if_addr,
        Err(e) => return Err(e.to_string()),
    };

    for ifa in ifaddr {
        match ifa.addr {
            IfAddr::V4(_) if interface_type == 4 && ifa.name == interface => {
                return Ok(ifa.ip().to_string())
            }
            IfAddr::V6(_) if interface_type == 6 && ifa.name == interface => {
                return Ok(ifa.ip().to_string())
            }
            _ => continue,
        }
    }

    return Err(format!(
        "No matching interfaces of name {} and type {}",
        interface, interface_type
    ));
}

pub fn ipv4(interface: &str) -> String {
    return match ip(interface, 4) {
        Ok(ip) => ip,
        Err(e) => e,
    };
}

pub fn ipv6(interface: &str) -> String {
    return match ip(interface, 6) {
        Ok(ip) => ip,
        Err(e) => e,
    };
}
