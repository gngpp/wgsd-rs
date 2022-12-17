use crate::args;

use crate::conf::endpoint::Node;
use crate::conf::{AsyncTryFrom, Configuration, NodeOpt};
use clap::ArgMatches;

use anyhow::Context;
use inquire::error::InquireResult;
use std::io::{Read, Write};
use std::path::PathBuf;

const PEER_TYPE: &str = "peer";
const PEER_SERVER_TYPE: &str = "peer-relay";

pub(crate) async fn subcommand_new_handler(
    _add_server: args::NewPeerRelayNetwork,
    _config: PathBuf,
) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) async fn subcommand_add_peer_handler(
    _add_peer: args::AddPeer,
    _config: PathBuf,
) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) async fn subcommand_revoke_peer_handler(_config: PathBuf) -> anyhow::Result<()> {
    Ok(())
}

pub(crate) async fn subcommand_print_peer_handler(_config: PathBuf) -> anyhow::Result<()> {
    Ok(())
}

fn print_and_qrcode(string: String) -> anyhow::Result<()> {
    let repeat_bounds = "-".repeat(70);
    println!(
        "generated configuration:\n{}\n{}\n{}\n",
        repeat_bounds, string, repeat_bounds
    );
    qr2term::print_qr(string).context("Failed to generate QRCode configuration")
}
