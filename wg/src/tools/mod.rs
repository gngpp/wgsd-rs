use std::io;

use ipnet::IpNet;

use crate::InterfaceName;

pub mod quick;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
use crate::tools::linux as platform;

#[cfg(target_os = "macos")]
use crate::tools::macos as platform;

pub(crate) fn set_address(interface: &InterfaceName, addr: IpNet) -> io::Result<()> {
    platform::set_addr(interface, addr)
}

pub(crate) fn set_up(interface: &InterfaceName, mtu: u32) -> io::Result<()> {
    platform::set_up(interface, mtu)
}

pub(crate) fn add_route(interface: &InterfaceName, cidr: IpNet) -> io::Result<bool> {
    platform::add_route(interface, cidr)
}
