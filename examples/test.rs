fn main() -> core::result::Result<(), String> {
    let addr = "::";
    let addr: std::net::IpAddr = addr
        .parse::<std::net::IpAddr>()
        .map_err(|_| format!("`{}` isn't a ip address", addr))?;
    if addr.is_ipv6() {
        println!("{} is ipv6", &addr);
    }
    if addr.is_ipv4() {
        println!("{} is ipv4", &&addr);
    }
    
    Ok(())
}
