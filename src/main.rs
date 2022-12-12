use crate::args::SubCommands;
use anyhow::anyhow;

mod args;
pub mod conf;
mod handler;
pub mod parser;
pub mod wg;

pub const DEFAULT_INTERFACE_ADDRESS: &str = "10.66.66.1/24, fd42:42:42::1/64";
pub const DEFAULT_INTERFACE_LISTEN_PORT: &str = "51820";
pub const DEFAULT_MTU: &str = "1420";
pub const DEFAULT_PEER_PERSISTENT_KEEPALIVE: &str = "21";
pub const DEFAULT_PEER_ENDPOINT_ALLOWED_IPS: &str = "10.66.66.0/24";

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let wgsdc = args::Wgsdc::parse();
    // enabled debug mode
    init_log(wgsdc.debug);
    match wgsdc.commands {
        Some(SubCommands::AddServer(add_interface)) => {
            handler::subcommand_add_server_handler(add_interface, wgsdc.config).await?
        }

        Some(SubCommands::AddPeer(add_peer)) => {
            handler::subcommand_add_peer_handler(add_peer, wgsdc.config).await?
        }

        Some(SubCommands::RevokePeer(revoke_peer)) => {
            handler::subcommand_revoke_peer_handler(revoke_peer, wgsdc.config).await?
        }

        Some(SubCommands::Config(conf)) => {
            handler::subcommand_config_handler(conf, wgsdc.config).await?
        }

        None => {}
    }
    Ok(())
}

fn init_log(debug: bool) {
    let log_level = if debug { "DEBUG" } else { "INFO" };
    std::env::set_var("RUST_LOG", log_level);
    use std::io::Write;
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}: {}",
                record.level(),
                //Format like you want to: <-----------------
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            )
        })
        .init();
}

fn sudo() -> anyhow::Result<()> {
    // root permission
    let _ = sudo::escalate_if_needed().map_err(|e| anyhow!(e.to_string()))?;
    Ok(())
}
