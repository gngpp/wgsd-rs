use colored::Colorize;
use wgctl::{AllowedIp, Backend, Device, PeerInfo};

#[cfg(target_os = "linux")]
const BACKEND: Backend = Backend::Kernel;
#[cfg(not(target_os = "linux"))]
const BACKEND: Backend = Backend::Userspace;

fn main() {
    let device_name_list = Device::list(BACKEND).unwrap();
    for ifname in device_name_list {
        let device = Device::get(&ifname, BACKEND).unwrap();
        print_device(&device)
    }
}

fn print_device(device: &Device) {
    println!(
        "{}: {}",
        "interface".green(),
        device.name.as_str_lossy().green()
    );
    if let Some(public_key) = &device.public_key {
        println!(
            "  {}: {}",
            "public key".white().bold(),
            public_key.to_base64()
        );
    }

    if let Some(_private_key) = &device.public_key {
        println!("  {}: {}", "private key".white().bold(), "(hidden)");
    }

    if let Some(listen_port) = device.listen_port {
        println!("  {}: {}", "listen port".white().bold(), listen_port);
    }

    for peer in &device.peers {
        println!();
        print_peer(peer);
    }
}

fn print_peer(peer: &PeerInfo) {
    println!(
        "{}: {}",
        "peer".yellow(),
        peer.config.public_key.to_base64().as_str().yellow()
    );
    if let Some(endpoint) = peer.config.endpoint {
        println!("  {}: {}", "endpoint".white().bold(), endpoint);
    }

    print!("  {}: ", "allowed ips".white().bold());
    for (i, allowed_ip) in peer.config.allowed_ips.iter().enumerate() {
        print_allowed_ip(allowed_ip);
        if i < peer.config.allowed_ips.len() - 1 {
            print!(", ");
        }
    }
    println!();
}

fn print_allowed_ip(allowed_ip: &AllowedIp) {
    print!("{}{}{}", allowed_ip.address, "/".cyan(), allowed_ip.cidr);
}
