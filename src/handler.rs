use clap::builder::TypedValueParser;
use clap::ArgMatches;
use std::io::{Read, Write};

pub(crate) fn command_client_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    if let Some(addr) = arg.get_one::<std::net::IpAddr>("client") {
        let port = *arg.get_one::<u16>("port").unwrap_or(&(0 as u16));
        let socket = std::net::SocketAddr::new(*addr, port);
        let mut connect = std::net::TcpStream::connect(socket)?;
        println!("connect to {}", connect.local_addr()?);

        connect.write(b"client")?;
    };
    Ok(())
}

pub(crate) fn command_server_handler(arg: &ArgMatches) -> anyhow::Result<()> {
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

pub(crate) fn subcommand_add_interface_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    if let Some(arg) = arg.subcommand_matches("add-interface") {
        if let Some(name) = arg.get_one::<String>("add-interface_name") {
            println!("#{}", name)
        }
        if let Some(address) = arg.get_one::<Vec<ipnet::IpNet>>("add-interface_address") {
            let str = address
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            println!("Address = {}", str.join(","));
        }
        if let Some(port) = arg.get_one::<u16>("add-interface_listen-port") {
            println!("ListenPort = {}", port);
        }
        if let Some(mtu) = arg.get_one::<u32>("add-interface_mtu") {
            println!("MTU = {}", mtu);
        }
    }
    Ok(())
}

pub(crate) fn subcommand_add_peer_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) fn subcommand_revoke_peer_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}
