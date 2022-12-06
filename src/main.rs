use crate::args::SubCommands;
use anyhow::anyhow;

mod args;
pub(crate) mod conf;
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
    let cli = args::CLI::parse();
    // enabled debug mode
    init_log(cli.debug);
    match cli.commands {
        Some(SubCommands::AddInterface(add_interface)) => {
            handler::subcommand_add_interface_handler(add_interface, cli.config).await?
        }

        Some(SubCommands::AddPeer(add_peer)) => {
            handler::subcommand_add_peer_handler(add_peer, cli.config).await?
        }

        Some(SubCommands::RevokePeer(revoke_peer)) => {
            handler::subcommand_revoke_peer_handler(revoke_peer, cli.config).await?
        }

        Some(SubCommands::Conf(conf)) => handler::subcommand_conf_handler(conf, cli.config).await?,

        Some(SubCommands::GenTemplate) => handler::subcommand_gen_template_handler().await?,

        None => {}
    }
    Ok(())
}

fn init_log(debug: bool) {
    if debug {
        std::env::set_var("RUST_LOG", "DEBUG");
    } else {
        std::env::set_var("RUST_LOG", "INFO");
    }
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
