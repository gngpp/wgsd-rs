use crate::{backends, InterfaceName};
use ipnet::IpNet;
use std::io;

#[cfg(target_os = "macos")]
fn command_handler(bin: &str, args: &[&str]) -> io::Result<std::process::Output> {
    let output = std::process::Command::new(bin).args(args).output()?;
    log::debug!("command: {} {}", bin, args.join(" "));
    log::debug!("status: {:?}", output.status.code());
    log::trace!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    log::trace!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if output.status.success() {
        Ok(output)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "failed to run {} {} command: {}",
                bin,
                args.join(" "),
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn set_addr(interface: &InterfaceName, addr: IpNet) -> io::Result<()> {
    let real_interface = backends::userspace::get_tun_name(interface)?;

    if matches!(addr, IpNet::V4(_)) {
        command_handler(
            "ifconfig",
            &[
                &real_interface,
                "inet",
                &addr.to_string(),
                &addr.addr().to_string(),
                "alias",
            ],
        )
        .map(|_output| ())
    } else {
        command_handler(
            "ifconfig",
            &[&real_interface, "inet6", &addr.to_string(), "alias"],
        )
        .map(|_output| ())
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn set_up(interface: &InterfaceName, mtu: u32) -> io::Result<()> {
    let real_interface = backends::userspace::get_tun_name(interface)?;
    command_handler("ifconfig", &[&real_interface, "mtu", &mtu.to_string()])?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn add_route(interface: &InterfaceName, cidr: IpNet) -> io::Result<bool> {
    let real_interface = backends::userspace::get_tun_name(interface)?;
    let output = command_handler(
        "route",
        &[
            "-q",
            "-n",
            "add",
            if matches!(cidr, IpNet::V4(_)) {
                "-inet"
            } else {
                "-inet6"
            },
            &cidr.to_string(),
            "-interface",
            &real_interface,
        ],
    )?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "failed to add route for device {} ({}): {}",
                &interface, real_interface, stderr
            ),
        ))
    } else {
        Ok(!stderr.contains("File exists"))
    }
}
