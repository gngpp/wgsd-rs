use std::io;

use ipnet::IpNet;

use crate::InterfaceName;

pub mod quick;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

pub(crate) fn set_address(interface: &InterfaceName, addr: IpNet) -> io::Result<()> {
    #[cfg(target_os = "linux")]
    crate::tools::linux::set_addr(interface, addr);

    #[cfg(target_os = "macos")]
    crate::tools::macos::set_addr(interface, addr)

}

pub(crate) fn set_up(interface: &InterfaceName, mtu: u32) -> io::Result<()> {
    #[cfg(target_os = "linux")]
    crate::tools::linux::set_up(interface, mtu)?;

    #[cfg(target_os = "macos")]
    crate::tools::macos::set_up(interface, mtu)
}

pub(crate) fn add_route(interface: &InterfaceName, cidr: IpNet) -> io::Result<bool> {
    #[cfg(target_os = "linux")]
    crate::tools::linux::add_route(interface, cidr);

    #[cfg(target_os = "macos")]
    crate::tools::macos::add_route(interface, cidr)
}