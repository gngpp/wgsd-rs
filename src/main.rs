use clap::{Arg, ArgAction, ArgMatches, Command};
use std::io::{Read, Write};
pub(crate) mod conf;
mod handler;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("wgsdc")
        .name("wgsdc")
        .version("1.0")
        .propagate_version(true)
        .subcommand_required(false)
        .arg_required_else_help(true)
        .author("gngpp. <gngppz@gmail.com>")
        .about("WireGuard configure the service discovery tool")
        .subcommands([
            Command::new("add-interface")
                .about("Add a new Interface")
                .propagate_version(true)
                .arg_required_else_help(true)
                .arg(
                    Arg::new("add-interface_name")
                        .long("name")
                        .value_name("name")
                        .action(ArgAction::Set)
                        .help("Interface's WireGuard name")
                        .required(true),
                ),
            Command::new("add-peer")
                .about("Add a new peer")
                .propagate_version(true)
                .arg_required_else_help(true)
                .arg(
                    Arg::new("add-peer_v4-addr")
                        .long("v4-addr")
                        .value_name("address")
                        .action(ArgAction::Set)
                        .help("Peer's WireGuard IPv4 Address"),
                )
                .arg(
                    Arg::new("add-peer_v6-addr")
                        .long("v6-addr")
                        .value_name("address")
                        .action(ArgAction::Set)
                        .help("Peer's WireGuard IPv4 Address"),
                )
                .arg(
                    Arg::new("tag")
                        .long("tag")
                        .action(ArgAction::Set)
                        .help("Peer tag name"),
                ),
            Command::new("revoke-peer")
                .about("Revoke existing peer")
                .propagate_version(true)
                .arg_required_else_help(true),
        ])
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
                .action(ArgAction::Set)
                .value_parser(conf::util::verify_host)
                .group("mode")
                .requires("port")
                .help("Run in client mode, connecting to <host>"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .value_parser(conf::util::verify_port_in_range)
                .help("Bind to a specific client/server port (TCP, temporary port by default)")
                .requires("client"),
        )
        .arg(
            Arg::new("token")
                .long("token")
                .help("Connect server token")
                .requires("client"),
        )
        .get_matches();

    handler::subcommand_add_interface_handler(&matches)?;
    handler::subcommand_add_peer_handler(&matches)?;
    handler::subcommand_revoke_peer_handler(&matches)?;

    client_handler(&matches)?;
    server_handler(&matches)?;
    Ok(())
}

fn client_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    if let Some(addr) = arg.get_one::<std::net::IpAddr>("client") {
        let port = *arg.get_one::<u16>("port").unwrap_or(&(0 as u16));
        let socket = std::net::SocketAddr::new(*addr, port);
        let mut connect = std::net::TcpStream::connect(socket)?;
        println!("connect to {}", connect.local_addr()?);

        connect.write(b"client")?;
    };
    Ok(())
}

fn server_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    match arg.get_one::<bool>("server") {
        None => {}
        Some(b) => {
            if *b {
                let port = *arg.get_one::<u16>("port").unwrap_or(&(0 as u16));
                let v4: std::net::Ipv4Addr = "0.0.0.0".parse::<std::net::Ipv4Addr>()?;
                let v6: std::net::Ipv6Addr = "::".parse::<std::net::Ipv6Addr>()?;
                let addrs = [
                    std::net::SocketAddr::from((std::net::IpAddr::V4(v4), port)),
                    std::net::SocketAddr::from((std::net::IpAddr::V6(v6), port)),
                ];
                let tcp_listen = std::net::TcpListener::bind(&addrs[..])?;
                println!("server listen to {}", tcp_listen.local_addr()?);
                loop {
                    for incoming in tcp_listen.accept() {
                        let mut tcp_stream = incoming.0;
                        let mut input = String::new();
                        let _ = tcp_stream.read_to_string(&mut input)?;
                        println!("{:?} says {}", incoming.1, input);
                    }
                }
            }
        }
    }
    Ok(())
}
