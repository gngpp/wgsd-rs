use std::ops::Deref;

extern crate ipnet;

pub struct WgConfig {
    interface: Interface,
    peer_list: Vec<Peer>,
}

impl WgConfig {
    pub fn new(interface: Interface, peer_list: Vec<Peer>) -> WgConfig {
        WgConfig {
            interface,
            peer_list,
        }
    }
    pub fn print(&self) {
        let mut lines: Vec<String> = Vec::new();
        lines.push(self.interface.tag());
        lines.push(String::from("[Interface]"));
        lines.push(format!("Address = {}", self.interface.address()));
        lines.push(format!("PrivateKey = {}", ""));
        lines.push(format!("ListenPort = {}", ""));
    }
}

pub struct Interface {
    tag: Option<String>,
    // address
    address: Option<Vec<ipnet::IpNet>>,
    // server's privatekey
    private_key: Option<String>,
    // server's listent port
    listen_port: Option<String>,

    mtu: Option<u32>,
    preup: Option<String>,
    postup: Option<String>,
    predown: Option<String>,
    postdown: Option<String>,
}

impl Interface {
    pub fn tag(&self) -> String {
        if let Some(ref tag) = self.tag {
            return format!("#{}", tag.to_string());
        }
        return String::new();
    }

    pub fn address(&self) -> String {
        if let Some(ref ipnet_address) = self.address {
            
        }
        String::new()
    }
}

pub struct Peer {
    pub tag: Option<String>,
    // server's public key
    pub public_key: Option<String>,
    // server's router allowed_ips
    pub allowed_ips: Option<Vec<ipnet::IpNet>>,
    // keep alive interval
    pub persistent_keepalive: Option<u16>,
    // peet endpiont
    pub endpoint: Option<PeerEndpoint>,
}

pub struct Node {
    pub tag: Option<String>,
    // address
    pub address: Option<Vec<ipnet::IpNet>>,
    // server's public key
    pub public_key: Option<String>,
    // server's privatekey
    pub private_key: Option<String>,
    // server's listent port
    pub listen_port: Option<String>,
    // server's router allowed_ips
    pub allowed_ips: Option<Vec<ipnet::IpNet>>,
    // keep alive interval
    pub persistent_keepalive: Option<u16>,
    // peet endpiont
    pub endpoint: Option<PeerEndpoint>,

    pub mtu: Option<u32>,

    pub preup: Option<String>,

    pub postup: Option<String>,

    pub predown: Option<String>,

    pub postdown: Option<String>,
}

impl Node {
    pub fn to_interface(self) -> Interface {
        Interface {
            tag: self.tag,
            address: self.address,
            private_key: self.private_key,
            listen_port: self.listen_port,
            mtu: self.mtu,
            preup: self.preup,
            postdown: self.postdown,
            predown: self.predown,
            postup: self.postup,
        }
    }

    pub fn to_peer(self) -> Peer {
        Peer {
            tag: self.tag,
            public_key: self.public_key,
            allowed_ips: self.allowed_ips,
            persistent_keepalive: self.persistent_keepalive,
            endpoint: self.endpoint,
        }
    }
}

#[derive(Debug)]
pub struct PeerEndpoint {
    address: std::net::IpAddr,
    port: u16,
}

impl PeerEndpoint {
    pub fn new(addr: std::net::IpAddr, port: u16) -> Self {
        Self {
            address: addr,
            port: port,
        }
    }
}

impl ToString for PeerEndpoint {
    fn to_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
