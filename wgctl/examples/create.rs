use wgctl::{Backend, Device, DeviceUpdate, KeyPair, PeerConfigBuilder};

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
        builder = builder.add_peer(PeerConfigBuilder::new(&keypair.public))
    }

    // interface alias
    let interface = "test".parse().unwrap();
    builder.apply(&interface, Backend::Userspace).unwrap();

    println!("create wireguard interfaces: {}", interface.to_string());

    let device = Device::get(&interface, Backend::Userspace).unwrap();

    for keypair in &keypair_list {
        assert!(device
            .peers
            .iter()
            .any(|p| p.config.public_key == keypair.public));
    }

}