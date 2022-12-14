use crate::args;

use crate::conf::endpoint::Node;
use crate::conf::{AsyncTryFrom, Configuration, RW};
use clap::ArgMatches;
use inquire::error::InquireResult;
use inquire::formatter::OptionFormatter;
use std::io::{Read, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

pub(crate) async fn subcommand_add_server_handler(
    _add_server: args::AddPeerServer,
    _config: String,
) -> anyhow::Result<()> {
    let mut configuration = Configuration::new(_config).await?;
    // set peer node
    configuration.set(Node::from(_add_server)).await?;
    configuration.print_std().await
}

pub(crate) async fn subcommand_add_peer_handler(
    _add_peer: args::AddPeer,
    _config: String,
) -> anyhow::Result<()> {
    let mut configuration = <Configuration as AsyncTryFrom<String>>::try_from(_config).await?;
    configuration.push(Node::from(_add_peer)).await?;
    configuration.print_std().await
}

pub(crate) async fn subcommand_revoke_peer_handler(config: String) -> anyhow::Result<()> {
    let mut configuration = <Configuration as AsyncTryFrom<String>>::try_from(config).await?;
    const PEER: &str = "peer";
    const PEER_SERVER: &str = "peer-server";
    let node_type_option = vec![PEER, PEER_SERVER];
    let node_type_select = inquire::Select::new(
        "select the peer node type that needs to be revoked.",
        node_type_option,
    )
    .prompt();
    match node_type_select {
        Ok(node_type) => match node_type {
            PEER => {
                let node_list = configuration.list().await?;
                let mut node_option = node_list.iter().map(|v| v.name()).collect::<Vec<&str>>();
                node_option.sort();
                let node_select = inquire::Select::new("select peer", node_option).prompt();
                match node_select {
                    Ok(node_name) => {
                        if configuration.remove_by_name(node_name).await.is_ok() {
                            configuration.print_std().await?;
                        };
                    }
                    Err(_) => {}
                }
            }
            PEER_SERVER => {
                let ans = inquire::Confirm::new("Are you sure you want to revoke the peer-server peer service node configuration?")
                        .with_default(false)
                        .with_help_message("This will clear the current configuration options")
                        .prompt();
                match ans {
                    Ok(true) => {
                        if configuration.clear().await.is_ok() {
                            configuration.print_std().await?;
                        }
                    }
                    Ok(false) => {
                        println!("Operation cancel")
                    }
                    Err(_) => {}
                }
            }
            _ => {}
        },

        Err(_) => {
            println!("There was an error, please try again")
        }
    }
    drop(configuration);
    Ok(())
}

pub(crate) async fn subcommand_config_handler(
    _conf: args::Config,
    _config: String,
) -> anyhow::Result<()> {
    // let configuration = Configuration::new(config).await?;
    // let wg = configuration.read().await?;
    // match _conf {
    //     Config { cat, sync: _ } => {
    //         if cat {
    //             if let Some(config_str) = wg.to_server_configuration_str()? {
    //                 println!("{}", config_str);
    //             }
    //
    //             if let Some(config_str) = wg.to_peer_configuration_str() {
    //                 println!("{}", config_str);
    //             }
    //         }
    //     }
    // }
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
