use crate::conf::model::Node;

use crate::args;
use crate::conf::Configuration;
use clap::ArgMatches;
use std::io::{Read, Write};
use std::num::ParseIntError;

pub(crate) async fn subcommand_add_server_handler(
    add_server: args::AddServer,
    config: String,
) -> anyhow::Result<()> {
    let configuration = Configuration::new(config).await?;
    // read configuration
    let mut wgsdc = configuration.read().await?;
    // set peer node
    wgsdc.node_server(Node::from(add_server));
    // write configuration
    configuration.write(wgsdc).await?;
    // print configuration to std
    configuration.print_std().await
}

pub(crate) async fn subcommand_add_peer_handler(
    add_peer: args::AddPeer,
    config: String,
) -> anyhow::Result<()> {
    let configuration = Configuration::new(config).await?;
    // read configuration
    let mut wgsdc = configuration.read().await?;
    // push peer list
    wgsdc.push_node(Node::from(add_peer));
    // write configuration
    configuration.write(wgsdc).await?;
    // print configuration to std
    configuration.print_std().await
}

pub(crate) async fn subcommand_revoke_peer_handler(
    revoke_peer: args::RevokePeer,
    config: String,
) -> anyhow::Result<()> {
    match revoke_peer {
        args::RevokePeer { shell, name } => {
            subcommand_revoke_peer_handler_inner(shell, name, config).await
        }
    }
}

async fn subcommand_revoke_peer_handler_inner(
    shell: bool,
    name: Option<String>,
    config: String,
) -> anyhow::Result<()> {
    let configuration = Configuration::new(config).await?;
    // read configuration
    let mut wgsdc = configuration.read().await?;
    let node_list = wgsdc.get_node_list();
    let mut modify = false;
    if shell {
        let format_print = |x: usize| {
            if x % 2 == 0 {
                "/"
            } else {
                "\\"
            }
        };
        println!("You can enter a serial number, or enters the 'exit' command");
        node_list
            .iter()
            .enumerate()
            .map(|(i, v)| format!("{} {} {}", i, format_print(i), v.name()))
            .for_each(|v| println!("{}", v));

        // Loops until the user enters the "exit" command
        loop {
            print!("revoke> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            // Perform actions based on user input
            match input.parse::<usize>() {
                Ok(index) => {
                    if index > node_list.len() - 1 {
                        println!("Unknown command: {}", input);
                    } else {
                        node_list.remove(index);
                        modify = true;
                        break;
                    }
                }
                Err(_) => {
                    match input {
                        "exit" => {
                            // exit shell
                            break;
                        }
                        _ => {
                            println!("Unknown command: {}", input);
                        }
                    }
                }
            }
        }
    }

    if let Some(name) = name {
        wgsdc.remove_node(name);
        modify = true;
    }

    if modify {
        configuration.write(wgsdc).await?;
        configuration.print_std().await?;
    }

    Ok(())
}

pub(crate) async fn subcommand_conf_handler(
    _conf: args::Conf,
    _config: String,
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
