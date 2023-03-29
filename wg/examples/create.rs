use std::net::{IpAddr, Ipv4Addr};
use wg::{AllowedIp, Backend, Device, DeviceUpdate, KeyPair, PeerConfigBuilder};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;
#[cfg(not(target_os = "macos"))]
const BACKEND: Backend = Backend::Userspace;

fn main() {
    if unsafe { libc::getuid() } != 0 {
        panic!("Please use sudo privileges")
    }

    let pair = KeyPair::generate();
    let mut builder = DeviceUpdate::new();

    builder = builder.set_keypair(pair).set_listen_port(51821);

    // add peer
    let keypair_list: Vec<_> = (0..2).map(|_| KeyPair::generate()).collect();
    for keypair in keypair_list.iter().enumerate() {
        let ipv4addr = Ipv4Addr::new(100, 100, (100 + keypair.0) as u8, 10);
        let peer_config_builder = PeerConfigBuilder::new(&keypair.1.public)
            .add_allowed_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 10, (2 + keypair.0) as u8)), 32)
            .add_allowed_ips(&[AllowedIp::new(IpAddr::V4(ipv4addr), 24)]);
        builder = builder.add_peer(peer_config_builder)
    }

    // interface alias name
    let interface_name = "test".parse().unwrap();
    builder.apply(&interface_name, BACKEND).unwrap();

    println!("create wireguard interfaces: {}", interface_name.to_string());

    let device = Device::get(&interface_name, Backend::Userspace).unwrap();

    for keypair in &keypair_list {
        assert!(device
            .peers
            .iter()
            .any(|p| p.config.public_key == keypair.public));
    }
}
