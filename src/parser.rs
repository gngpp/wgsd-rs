use anyhow::anyhow;
use std::path::PathBuf;

// address parser
pub(crate) fn parser_address(s: &str) -> anyhow::Result<std::net::IpAddr> {
    let addr = s
        .parse::<std::net::IpAddr>()
        .map_err(|_| anyhow!(format!("`{}` isn't a ip address", s)))?;
    Ok(addr)
}

// host parser
pub(crate) fn parser_host(s: &str) -> anyhow::Result<String> {
    let address = parser_address(s);
    return match address {
        Ok(addr) => {
            Ok(addr.to_string())
        }
        Err(e) => {
            Err(e)
        }
    }
}


const PORT_RANGE: std::ops::RangeInclusive<usize> = 1024..=65535;

// port range parser
pub(crate) fn parser_port_in_range(s: &str) -> anyhow::Result<u16> {
    let port: usize = s
        .parse()
        .map_err(|_| anyhow!(format!("`{}` isn't a port number", s)))?;
    if PORT_RANGE.contains(&port) {
        return Ok(port as u16);
    }
    anyhow::bail!(format!(
        "Port not in range {}-{}",
        PORT_RANGE.start(),
        PORT_RANGE.end()
    ))
}

// address list range parser
pub(crate) fn parser_address_in_range(s: &str) -> anyhow::Result<Vec<ipnet::IpNet>> {
    let vec: Vec<&str> = s.split(",").map(|v| v.trim()).collect();
    let mut res = Vec::new();
    for value in &vec {
        let address = value.parse::<ipnet::IpNet>()?;
        res.push(address)
    }
    Ok(res)
}

pub(crate) fn parser_mtu(s: &str) -> anyhow::Result<u16> {
    let mtu = s
        .parse::<u16>()
        .map_err(|_| anyhow!(format!("`{}` isn't a mtu number", s)))?;
    Ok(mtu)
}

pub(crate) fn parser_conf(s: &str) -> anyhow::Result<PathBuf> {
    anyhow::bail!("implement me")
}
