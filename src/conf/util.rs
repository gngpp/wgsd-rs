pub(crate) fn verify_host(s: &str) -> Result<std::net::IpAddr, String> {
    let addr: std::net::IpAddr = s
        .parse::<std::net::IpAddr>()
        .map_err(|_| format!("`{}` isn't a ip address", s))?;
    Ok(addr)
}

const PORT_RANGE: std::ops::RangeInclusive<usize> = 1024..=65535;

pub(crate) fn verify_port_in_range(s: &str) -> Result<u16, String> {
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", s))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "Port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}
