extern crate core;

use crate::args::SubCommands;
use anyhow::anyhow;

mod args;
mod conf;
mod handler;
mod parser;
mod wg;
pub mod model;
pub mod standard;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    use clap::Parser;
    let wgsdc = args::Opt::parse();
    // enabled debug mode
    init_log(wgsdc.debug);
    // match wgsdc.commands {
    //     Some(SubCommands::New(add_interface)) => {
    //         handler::subcommand_new_handler(add_interface, wgsdc.dir).await?
    //     }
    //
    //     Some(SubCommands::AddPeer(add_peer)) => {
    //         handler::subcommand_add_peer_handler(add_peer, wgsdc.dir).await?
    //     }
    //
    //     Some(SubCommands::RevokePeer) => {
    //         handler::subcommand_revoke_peer_handler(wgsdc.dir).await?
    //     }
    //
    //     Some(SubCommands::PrintPeer) => {
    //         handler::subcommand_print_peer_handler(wgsdc.dir).await?;
    //     }
    //
    //     None => {}
    // }
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
