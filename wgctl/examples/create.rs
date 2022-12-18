use std::net::{IpAddr, Ipv4Addr};
use wgctl::{AllowedIp, Backend, Device, DeviceUpdate, KeyPair, PeerConfigBuilder};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;


fn main() {
    if unsafe { libc::getuid() } != 0 {
        panic!("Please use sudo privileges")
    }

    let pair = KeyPair::generate();
    let mut builder = DeviceUpdate::new();

    builder = builder.set_private_key(pair.private)
        .set_listen_port(51821);

    // add peer
    let keypair_list: Vec<_> = (0..2).map(|_| KeyPair::generate()).collect();
    for keypair in &keypair_list {
        let ipv4addr = Ipv4Addr::new(100,100,100, 10);
        let peer_config_builder = PeerConfigBuilder::new(&keypair.public)
            .add_allowed_ip(IpAddr::V4(Ipv4Addr::new(192, 168,10,2)), 32)
            .add_allowed_ips(&[AllowedIp::new(IpAddr::V4(ipv4addr), 24)]);
        builder = builder.add_peer(peer_config_builder)
    }

    // interface alias
    let interface = "test".parse().unwrap();
    builder.apply(&interface, BACKEND).unwrap();

    println!("create wireguard interfaces: {}", interface.to_string());

    let device = Device::get(&interface, Backend::Userspace).unwrap();

    for keypair in &keypair_list {
        assert!(device
            .peers
            .iter()
            .any(|p| p.config.public_key == keypair.public));
    }

}