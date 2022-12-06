use crate::parser;
use crate::{
    DEFAULT_INTERFACE_ADDRESS, DEFAULT_INTERFACE_LISTEN_PORT, DEFAULT_MTU,
    DEFAULT_PEER_ENDPOINT_ALLOWED_IPS, DEFAULT_PEER_PERSISTENT_KEEPALIVE,
};
use clap::{Args, Subcommand};
use crate::conf::model::IpNet;

#[derive(clap::Parser)]
#[command(about, version, author, arg_required_else_help = true)]
#[command(propagate_version = true)]
pub(crate) struct CLI {
    /// Enable debug mode
    #[arg(long)]
    pub debug: bool,

    /// Run in server mode
    #[arg(short, long, group = "wgsdc", requires = "config")]
    pub server: bool,

    /// Run in client mode, connecting to <host>
    #[arg(short, long, value_parser = parser::parser_host, value_name = "HOST", group = "wgsdc", requires = "config")]
    pub client: Option<String>,

    /// Bind to a specific client/server port (TCP, temporary port by 1024-65535)
    #[arg(short, long, value_parser = parser::parser_port_in_range, default_value = "8888")]
    pub port: Option<u16>,

    /// Client/Server connect Token
    #[arg(long)]
    pub token: Option<String>,

    /// Use configuration
    #[arg(global = true, default_value = "wg0")]
    pub config: String,

    /// Subcommands
    #[command(subcommand)]
    pub commands: Option<SubCommands>,
}

#[derive(Subcommand)]
pub(crate) enum SubCommands {
    /// Add WireGuard Interface
    #[command(arg_required_else_help = true)]
    AddInterface(AddInterface),

    /// Add WireGuard Peer
    #[command(arg_required_else_help = true)]
    AddPeer(AddPeer),

    /// Revoke WireGuard existing peer
    #[command(arg_required_else_help = true)]
    RevokePeer(RevokePeer),

    /// Generate a WireGuard configuration template
    GenTemplate,

    /// WireGuard Configuration
    #[command(arg_required_else_help = true)]
    Conf(Conf),
}

#[allow(unused_qualifications)]
#[derive(Args)]
pub(crate) struct AddInterface {
    /// Interface description
    #[arg(long, short)]
    pub description: Option<String>,

    /// Interface's WireGuard Peer Endpoint address/domain
    #[arg(long, value_name = "HOST", value_parser = parser::parser_host)]
    pub endpoint: String,

    /// Interface's WireGuard address
    // issue https://github.com/clap-rs/clap/issues/4481#issuecomment-1314475143
    #[arg(long, default_value = DEFAULT_INTERFACE_ADDRESS, value_parser = parser::parser_address_in_range)]
    pub address: std::vec::Vec<IpNet>,

    /// Interface's WireGuard listen port
    #[arg(long, default_value = DEFAULT_INTERFACE_LISTEN_PORT, value_parser = parser::parser_port_in_range)]
    pub listen_port: u16,

    /// Interface's WireGuard MTU
    #[arg(long, default_value = DEFAULT_MTU, value_parser = parser::parser_mtu)]
    pub mtu: u16,

    /// Interface's WireGuard PostUp command
    #[arg(long)]
    pub post_up: Option<String>,

    /// Interface's WireGuard PostDown command
    #[arg(long)]
    pub post_down: Option<String>,

    /// Interface's WireGuard PreUp command
    #[arg(long)]
    pub pre_up: Option<String>,

    /// Interface's WireGuard PreDown command
    #[arg(long)]
    pub pre_down: Option<String>,
}

#[allow(unused_qualifications)]
#[derive(Args)]
pub(crate) struct AddPeer {
    /// Peer description
    #[arg(long, short)]
    pub description: Option<String>,

    /// Peer name
    #[arg(long, short)]
    pub name: String,

    /// Peer AllowedIPs
    #[arg(long, value_parser = parser::parser_address_in_range)]
    pub allowed_ips: std::vec::Vec<IpNet>,

    /// Peer MTU
    #[arg(long, default_value = DEFAULT_MTU)]
    pub mtu: u16,

    /// Peer persistent keepalive
    #[arg(long, default_value = DEFAULT_PEER_PERSISTENT_KEEPALIVE)]
    pub persistent_keepalive: u16,

    /// Peer endpoint allowed ips
    #[arg(long, value_name = "ALLOWED_IPS", default_value = DEFAULT_PEER_ENDPOINT_ALLOWED_IPS, value_parser = parser::parser_address_in_range)]
    pub endpoint_allowed_ips: std::vec::Vec<IpNet>,
}

#[derive(Args)]
pub(crate) struct RevokePeer {
    /// Peer name
    #[arg(long, short)]
    pub name: String,
}

#[derive(Args)]
pub(crate) struct Conf {
    /// Print WireGuard configuration
    #[arg(long)]
    pub cat: bool,
    /// Sync WireGuard configuration
    #[arg(long)]
    pub sync: bool,
}
