use crate::conf::model::{Endpoint, Node};
use crate::conf::Configuration;
use crate::{args, wg};

use clap::ArgMatches;
use std::io::{Read, Write};

pub(crate) async fn subcommand_add_interface_handler(
    add_interface: args::AddInterface,
    _config: String,
) -> anyhow::Result<()> {
    let mut node = Node::default();
    let key_pair = wg::WireGuardCommand::generate_key_pair(false)?;
    node.set_is_server(true)
        .set_description(add_interface.description)
        .set_endpoint(Some(Endpoint::new(
            add_interface.endpoint,
            add_interface.listen_port,
        )))
        .set_address(Some(add_interface.address))
        .set_listen_port(Some(add_interface.listen_port))
        .set_mtu(Some(add_interface.mtu))
        .set_post_up(add_interface.post_up)
        .set_post_down(add_interface.post_down)
        .set_pre_up(add_interface.pre_up)
        .set_pre_down(add_interface.pre_down)
        .set_public_key(Some(key_pair.public_key().to_string()))
        .set_private_key(Some(key_pair.private_key().to_string()));
    println!("{:#?}", node);
    Ok(())
}

pub(crate) async fn subcommand_add_peer_handler(
    _add_peer: args::AddPeer,
    _config: String,
) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) async fn subcommand_revoke_peer_handler(
    _revoke_peer: args::RevokePeer,
    _config: String,
) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) async fn subcommand_conf_handler(
    _conf: args::Conf,
    config: String,
) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) async fn subcommand_gen_template_handler() -> anyhow::Result<()> {
    Ok(())
}

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
