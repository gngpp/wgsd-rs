use clap::{Arg, ArgAction, Command};

fn main() -> anyhow::Result<()> {
    let matches = Command::new("wgsd-rs")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("WireGuard configure the service discovery tool")
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .action(ArgAction::SetTrue)
                .group("mode")
                .help("Run in server mode"),
        )
        .arg(
            Arg::new("client")
                .short('c')
                .long("client")
                .value_name("host")
                .value_parser(verify_host)
                .group("mode")
                .help("Run in client mode, connecting to <host>"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_parser(verify_port_in_range)
                .help("Bind to a specific client/server port (TCP, temporary port by default)")
                .required(true),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let port = *matches
        .get_one::<u16>("port")
        .expect("'port' is required and parsing will fail if its missing");

    match matches.get_one::<std::net::IpAddr>("client") {
        Some(addr) => {
            if addr.is_ipv6() {
                println!("{} is ipv6", addr.to_string());
            }
            if addr.is_ipv4() {
                println!("{} is ipv4", addr.to_string());
            }
        }
        None => {
            let v4: std::net::Ipv4Addr = "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap();
            let v6: std::net::Ipv6Addr = "::".parse::<std::net::Ipv6Addr>().unwrap();
            println!("{}", v6.to_ipv4_mapped().unwrap().to_string());
            let addrs = [
                std::net::SocketAddr::from((std::net::IpAddr::V4(v4), port)),
                std::net::SocketAddr::from((std::net::IpAddr::V6(v6), port)),
            ];
            let v4 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(0, 0, 0, 0), port);
            let tcp = std::net::TcpListener::bind(&addrs[..])?;
            println!("server listent to {}", v4.to_string());
            for _t in tcp.incoming() {}
        }
    };
    Ok(())
}

fn verify_host(s: &str) -> Result<std::net::IpAddr, String> {
    let addr: std::net::IpAddr = s
        .parse::<std::net::IpAddr>()
        .map_err(|_| format!("`{}` isn't a ip address", s))?;
    Ok(addr)
}

const PORT_RANGE: std::ops::RangeInclusive<usize> = 1024..=65535;

fn verify_port_in_range(s: &str) -> Result<u16, String> {
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
