use clap::ArgMatches;
use clap::builder::TypedValueParser;

pub(crate) fn subcommand_add_interface_handler(arg: &ArgMatches) -> anyhow::Result<()> {
    if let Some(arg) = arg.subcommand_matches("add-interface") {
        if let Some(name) = arg.get_one::<String>("add-interface_name") {
            println!("{}", name)
        }
    }
    Ok(())
}

pub(crate) fn subcommand_add_peer_handler(arg: &ArgMatches) -> anyhow::Result<()> {

    Ok(())
}

pub(crate) fn subcommand_revoke_peer_handler(arg: &ArgMatches) -> anyhow::Result<()> {

    Ok(())
}