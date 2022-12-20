use std::net::{IpAddr, Ipv4Addr};
use ipnet::IpNet;
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
    let mut builder = wgctl::tools::quick::WgQuick::new("test").unwrap();
    builder = builder.set_keypair(pair)
        .set_address("192.168.10.1/24".parse().unwrap())
        .set_listen_port(51822);


    builder.apply(BACKEND).unwrap();

    println!("create wireguard interfaces: {}", "test");

    // let device = Device::get(&interface, Backend::Userspace).unwrap();
    //
    // for keypair in &keypair_list {
    //     assert!(device
    //         .peers
    //         .iter()
    //         .any(|p| p.config.public_key == keypair.public));
    // }
}
