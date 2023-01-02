use anyhow::anyhow;
use ipnet::IpNet;

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
        Ok(addr) => Ok(addr.to_string()),
        Err(_) => {
            let vec = s.split('.').map(|x| x.trim()).collect::<Vec<&str>>();
            // Part of the domain name rules, I don't think anyone will deliberately mistake the domain name, right?
            if s.is_empty()
                || s.chars().count() > 253
                || vec.len() <= 1
                || s.starts_with('-')
                || s.ends_with('-')
                || s.contains('*')
            {
                anyhow::bail!("{} does not conform to domain specification!", s)
            }
            Ok(String::from(s))
        }
    };
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
pub(crate) fn parser_address_in_range(s: &str) -> anyhow::Result<Vec<IpNet>> {
    let vec = s.split(",").map(|v| v.trim()).collect::<Vec<&str>>();
    let mut res = Vec::new();
    for value in &vec {
        res.push(value.parse::<IpNet>()?)
    }
    Ok(res)
}

pub(crate) fn parser_mtu(s: &str) -> anyhow::Result<u16> {
    let mtu = s
        .parse::<u16>()
        .map_err(|_| anyhow!(format!("`{}` isn't a mtu number", s)))?;
    Ok(mtu)
}
