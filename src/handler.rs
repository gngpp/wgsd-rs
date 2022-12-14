use crate::args;

use crate::conf::endpoint::Node;
use crate::conf::{Configuration, RW};
use clap::ArgMatches;
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
    let mut configuration = Configuration::new(_config).await?;
    configuration.push(Node::from(_add_peer)).await?;
    configuration.print_std().await
}

pub(crate) async fn subcommand_revoke_peer_handler(config: String) -> anyhow::Result<()> {
    let mut configuration = Configuration::new(config).await?;
    // read configuration
    let node_list = configuration.list().await?;
    let mut modify = false;
    let format_print = |x: usize| if x % 2 == 0 { "/" } else { "\\" };
    // Loops until the user enters the "exit" command
    let mut stdout = tokio::io::stdout();
    stdout
        .write_all(b"You can enter a serial number select the revoke peer type, or enters the 'exit' command.\n")
        .await?;
    stdout.flush().await?;

    ["peer", "peer server"]
        .iter()
        .enumerate()
        .for_each(|(i, v)| println!("{} {} {}", i, format_print(i), v));

    stdout.write_all(b"revoke> ").await?;
    stdout.flush().await?;

    // stdin input command
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    // command
    let input = stdin.next_line().await?.unwrap();

    match input.as_str() {
        "exit" => {
            return Ok(());
        }
        "0" => {
            println!("You can enter a serial number or a name, or enters the 'exit' command.");
            node_list
                .iter()
                .enumerate()
                .for_each(|(i, v)| println!("{} {} {}", i, format_print(i), v.name()));

            loop {
                stdout.write_all(b"revoke> ").await?;
                stdout.flush().await?;

                let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
                let input = stdin.next_line().await?.unwrap();
                // Perform actions based on user input
                match input.parse::<usize>() {
                    Ok(index) => match configuration.remove_by_index(index).await {
                        Ok(_) => {
                            modify = true;
                            break;
                        }
                        Err(err) => {
                            println!("Error: {}", err.to_string())
                        }
                    },
                    Err(_) => {
                        match input.as_str() {
                            "exit" => {
                                // exit shell
                                break;
                            }
                            _ => match configuration.remove_by_name(input.as_str()).await {
                                Ok(_) => {
                                    modify = true;
                                    break;
                                }
                                Err(err) => {
                                    println!("Error: {}", err.to_string())
                                }
                            },
                        }
                    }
                }
            }

            if modify {
                configuration.print_std().await?;
            }
        }
        "1" => {
            configuration.drop().await?;
        }
        _ => {
            println!("Unknown command: {}", input)
        }
    }

    drop(stdin);
    drop(stdout);
    drop(node_list);
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
