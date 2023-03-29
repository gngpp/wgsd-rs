use ipnet::IpNet;
use std::net::{IpAddr, Ipv4Addr};
use wg::{AllowedIp, Backend, Device, DeviceUpdate, InterfaceName, KeyPair, PeerConfigBuilder};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

fn main() {
    if unsafe { libc::getuid() } != 0 {
        panic!("Please use sudo privileges")
    }

    let name = "test";

    let pair = KeyPair::generate();
    let mut builder = wg::tools::quick::WgQuick::new(name).unwrap();
    builder = builder
        .set_keypair(pair)
        .set_address("192.168.10.1/24".parse().unwrap())
        .set_listen_port(51822);

    builder.apply(BACKEND).unwrap();

    println!("create wireguard interfaces: {}", name);

    let result = name.parse::<InterfaceName>().unwrap();
    let device = Device::get(&result, Backend::Userspace).unwrap();

    device.print().unwrap();
}
