use std::net::IpAddr;
use std::str::FromStr;

/// 解析 IP:ADDR 并返回处理结果
pub fn validate_addr(addr: &str) -> Result<(IpAddr, u16), String> {
    let parts: Vec<&str> = addr.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid address format. Use IP:PORT".to_string());
    }

    let ip = parts[0];
    let port = parts[1];

    // 解析并验证 IP 地址
    let ip_addr = IpAddr::from_str(ip).map_err(|_| "Invalid IP address".to_string())?;

    // 解析并验证端口号
    let port_num = port
        .parse::<u16>()
        .map_err(|_| "Invalid port number".to_string())?;
    if port_num == 0 {
        return Err("Port number cannot be 0".to_string());
    }

    Ok((ip_addr, port_num))
}
