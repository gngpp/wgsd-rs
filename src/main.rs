use clap::Subcommand;

pub(crate) mod conf;
mod handler;
pub mod parser;

pub const DEFAULT_INTERFACE_ADDRESS: &str = "10.66.66.1/24, fd42:42:42::1/64";
pub const DEFAULT_INTERFACE_LISTEN_PORT: &str = "51820";
pub const DEFAULT_INTERFACE_MTU: &str = "1420";

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
    #[arg(short, long, value_parser = parser::parser_host, value_name = "host", group = "wgsdc", requires = "token")]
    client: Option<std::net::IpAddr>,

    /// Bind to a specific client/server port (TCP, temporary port by 1024-65535)
    #[arg(short, long, value_parser = parser::parser_port_in_range, value_name = "port", default_value = "8888")]
    port: u16,

    /// Client/Server connect Token
    #[arg(long, value_name = "token")]
    token: Option<String>,

    /// Subcommands
    #[command(subcommand)]
    subcommand: Option<SubCommands>,

    /// Use configuration
    config: String,
}

#[derive(Subcommand)]
enum SubCommands {
    /// WireGuard configuration
    #[command(arg_required_else_help = true)]
    INTERFACE {
        /// lists config
        #[arg(short, long)]
        list: bool
    }
}

fn main() -> anyhow::Result<()> {
    // let matches = Command::new("wgsdc")
    //     .name("wgsdc")
    //     .version("1.0")
    //     .propagate_version(true)
    //     .subcommand_required(false)
    //     .arg_required_else_help(true)
    //     .author("gngpp. <gngppz@gmail.com>")
    //     .about("WireGuard configure the service discovery tool")
    //     .subcommands([
    //         Command::new("add-interface")
    //             .about("Add a new Interface")
    //             .propagate_version(true)
    //             .arg_required_else_help(true)
    //             .arg(
    //                 Arg::new("add-interface_name")
    //                     .long("name")
    //                     .short('n')
    //                     .value_name("name")
    //                     .help("Interface's WireGuard name")
    //                     .required(true),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_address")
    //                     .long("address")
    //                     .value_name("address")
    //                     .default_value(DEFAULT_INTERFACE_ADDRESS)
    //                     .value_parser(parser::parser_address_in_range)
    //                     .help("Interface's WireGuard address"),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_listen-port")
    //                     .long("listen-port")
    //                     .value_name("port")
    //                     .default_value(DEFAULT_INTERFACE_LISTEN_PORT)
    //                     .value_parser(parser::parser_port_in_range)
    //                     .help("Interface's WireGuard listen port"),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_mtu")
    //                     .long("mtu")
    //                     .value_name("mtu")
    //                     .default_value(DEFAULT_INTERFACE_MTU)
    //                     .value_parser(parser::parser_mtu)
    //                     .help("Interface's WireGuard mtu"),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_post-up")
    //                     .long("post-up")
    //                     .value_name("command")
    //                     .help("Interface's WireGuard PostUp command"),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_post-down")
    //                     .long("post-down")
    //                     .value_name("command")
    //                     .help("Interface's WireGuard PostDown command"),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_pre-up")
    //                     .long("pre-up")
    //                     .value_name("command")
    //                     .help("Interface's WireGuard PreUp command"),
    //             )
    //             .arg(
    //                 Arg::new("add-interface_pre-down")
    //                     .long("pre-down")
    //                     .value_name("command")
    //                     .help("Interface's WireGuard PreDown command"),
    //             ),
    //         Command::new("add-peer")
    //             .about("Add a new peer")
    //             .propagate_version(true)
    //             .arg_required_else_help(true)
    //             .arg(
    //                 Arg::new("add-peer_v4-addr")
    //                     .long("v4-addr")
    //                     .value_name("address")
    //                     .help("Peer's WireGuard IPv4 Address"),
    //             )
    //             .arg(
    //                 Arg::new("add-peer_v6-addr")
    //                     .long("v6-addr")
    //                     .value_name("address")
    //                     .help("Peer's WireGuard IPv4 Address"),
    //             )
    //             .arg(Arg::new("tag").long("tag").help("Peer tag name")),
    //         Command::new("revoke-peer")
    //             .about("Revoke existing peer")
    //             .propagate_version(true)
    //             .arg_required_else_help(true),
    //     ])
    //     .get_matches();

    // handler::subcommand_add_interface_handler(&matches)?;
    // handler::subcommand_add_peer_handler(&matches)?;
    // handler::subcommand_revoke_peer_handler(&matches)?;

    // handler::command_client_handler(&matches)?;
    // handler::command_server_handler(&matches)?;
    use clap::Parser;
    let cli = CLI::parse();
    // enabled debug mode
    init_log(cli.debug);
    log::debug!("address");
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
