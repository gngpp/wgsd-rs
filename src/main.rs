use clap::{Args, Subcommand};

pub(crate) mod conf;
mod handler;
pub mod parser;

pub const DEFAULT_INTERFACE_ADDRESS: &str = "10.66.66.1/24, fd42:42:42::1/64";
pub const DEFAULT_INTERFACE_LISTEN_PORT: &str = "51820";
pub const DEFAULT_MTU: &str = "1420";
pub const DEFAULT_PEER_PERSISTENT_KEEPALIVE: &str = "21";
pub const DEFAULT_PEER_ENDPOINT_ALLOWED_IPS: &str = "10.66.66.0/24";

#[derive(clap::Parser)]
#[command(about, version, author, arg_required_else_help = true)]
#[command(propagate_version = true)]
struct CLI {
    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,

    /// Run in server mode
    #[arg(short, long, group = "wgsdc", requires = "token")]
    server: bool,

    /// Run in client mode, connecting to <host>
    #[arg(short, long, value_parser = parser::parser_host, value_name = "HOST", group = "wgsdc", requires = "token")]
    client: Option<std::net::IpAddr>,

    /// Bind to a specific client/server port (TCP, temporary port by 1024-65535)
    #[arg(short, long, value_parser = parser::parser_port_in_range, default_value = "8888")]
    port: Option<u16>,

    /// Client/Server connect Token
    #[arg(long)]
    token: Option<String>,

    /// Use configuration
    config: Option<String>,

    /// Subcommands
    #[command(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Subcommand)]
#[command(next_line_help = false)]
enum SubCommands {
    /// Add WireGuard Interface
    #[command(arg_required_else_help = true)]
    AddInterface(AddInterface),

    /// Add WireGuard Peer
    #[command(arg_required_else_help = true)]
    AddPeer(AddPeer),

    /// Revoke WireGuard existing peer
    #[command(arg_required_else_help = true)]
    RevokePeer(RevokePeer),

    /// WireGuard Configuration
    #[command(arg_required_else_help = true)]
    Conf(Conf),
}

#[allow(unused_qualifications)]
#[derive(Args)]
struct AddInterface {
    /// Interface name
    #[arg(long)]
    name: String,

    /// Interface's WireGuard Peer Endpoint address
    #[arg(long, value_name = "ADDRESS", value_parser = parser::parser_host)]
    endpoint: String,

    /// Interface's WireGuard address
    // issue https://github.com/clap-rs/clap/issues/4481#issuecomment-1314475143
    #[arg(long, default_value = DEFAULT_INTERFACE_ADDRESS, value_parser = parser::parser_address_in_range)]
    address: std::vec::Vec<ipnet::IpNet>,

    /// Interface's WireGuard listen port
    #[arg(long, default_value = DEFAULT_INTERFACE_LISTEN_PORT, value_parser = parser::parser_port_in_range)]
    listen_port: u16,

    /// Interface's WireGuard MTU
    #[arg(long, default_value = DEFAULT_MTU, value_parser = parser::parser_mtu)]
    mtu: u16,

    /// Interface's WireGuard PostUp command
    #[arg(long)]
    post_up: Option<String>,

    /// Interface's WireGuard PostDown command
    #[arg(long)]
    post_down: Option<String>,

    /// Interface's WireGuard PreUp command
    #[arg(long)]
    pre_up: Option<String>,

    /// Interface's WireGuard PreDown command
    #[arg(long)]
    pre_down: Option<String>,

}

#[allow(unused_qualifications)]
#[derive(Args)]
struct AddPeer {

    /// Peer name
    #[arg(long)]
    name: String,

    /// Peer AllowedIPs
    #[arg(long, value_parser = parser::parser_address_in_range)]
    allowed_ips: std::vec::Vec<ipnet::IpNet>,

    /// Peer MTU
    #[arg(long, default_value = DEFAULT_MTU)]
    mtu: u16,

    /// Peer persistent keepalive
    #[arg(long, default_value = DEFAULT_PEER_PERSISTENT_KEEPALIVE)]
    persistent_keepalive: u16,

    /// Peer endpoint allowed ips
    #[arg(long, value_name = "ALLOWED_IPS", default_value = DEFAULT_PEER_ENDPOINT_ALLOWED_IPS, value_parser = parser::parser_address_in_range)]
    endpoint_allowed_ips: std::vec::Vec<ipnet::IpNet>,

}

#[derive(Args)]
struct RevokePeer {


}

#[derive(Args)]
struct Conf {
    // Lists configuration
    #[arg(short, long)]
    list: bool,

    // Sync configuration
    sync: bool,
}

fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let cli = CLI::parse();
    // enabled debug mode
    init_log(cli.debug);
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
