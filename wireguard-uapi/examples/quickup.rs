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
    let mut wgquick_builder = wg::tools::quick::WgQuick::new(name).unwrap();
    wgquick_builder = wgquick_builder
        .set_keypair(pair)
        .set_address("192.168.10.1/24".parse().unwrap())
        .set_listen_port(51822);

    // add peer
    let keypair_list: Vec<_> = (0..2).map(|_| KeyPair::generate()).collect();
    for keypair in keypair_list.iter().enumerate() {
        let ipv4addr = Ipv4Addr::new(100, 100, (100 + keypair.0) as u8, 10);
        let peer_config_builder = PeerConfigBuilder::new(&keypair.1.public)
            .add_allowed_ip(
                IpAddr::V4(Ipv4Addr::new(192, 168, 10, (2 + keypair.0) as u8)),
                32,
            )
            .add_allowed_ips(&[AllowedIp::new(IpAddr::V4(ipv4addr), 24)]);
        wgquick_builder = wgquick_builder.add_peer(peer_config_builder)
    }

    wgquick_builder.apply(BACKEND).unwrap();

    println!("create wireguard interfaces: {}", name);

    let result = name.parse::<InterfaceName>().unwrap();
    let device = Device::get(&result, Backend::Userspace).unwrap();

    device.print().unwrap();
}
