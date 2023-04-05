extern crate core;

use std::{path::PathBuf, str::FromStr};

use anyhow::anyhow;
use dirs::home_dir;
use sea_orm::{ActiveValue, Database, EntityTrait};

mod args;
mod conf;
pub mod db;
mod handler;
pub mod model;
mod parser;
pub mod standard;
mod wg;

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    let test = dirs::home_dir().unwrap().join("db");
    let db_path = test.as_path();
    println!("{}", db_path.display());
   
    
    db::initialize_database(test.clone()).await?;

    let node = db::model::node_relay::ActiveModel {
        name: ActiveValue::Set("HappyTest".to_owned()),
        relay: ActiveValue::Set(true),
        ..Default::default()
    };
    let db = sea_orm::Database::connect(format!("sqlite:{}", test.display())).await?;
    let result = db::model::prelude::NodeRelay::insert(node).exec(&db).await?;
    println!("{:?}", result);

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
