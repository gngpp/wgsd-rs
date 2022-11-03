use std::fmt::format;
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
    pub fn get_config(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        lines.push(self.interface.tag());
        lines.push(String::from("[Interface]"));
        lines.push(format!("Address = {}", self.interface.address()));
        lines.push(format!("PrivateKey = {}", self.interface.private_key()));
        lines.push(format!("ListenPort = {}", self.interface.listen_port()));
        lines.push(format!("MTU = {}\n", self.interface.mtu()));

        for peer in self.peer_list.iter() {
            lines.push(peer.tag());
            lines.push(String::from("[Peer]"));
            lines.push(format!("PublicKey = {}", peer.public_key()));
            lines.push(format!("AllowedIPs = {}", peer.allowed_ips()));
            lines.push(format!("Endpoint = {}", peer.endpoint()));
            lines.push(format!(
                "PersistentKeepalive = {}\n",
                peer.persistent_keepalive()
            ));
        }
        lines
    }
}

pub struct Interface {
    // node mode (client/server)
    is_server: bool,
    // tag name
    tag: Option<String>,
    // interface WireGuard address
    address: Option<Vec<ipnet::IpNet>>,
    // interface private key
    private_key: Option<String>,
    // interface listen port
    listen_port: Option<String>,
    // interface MTU
    mtu: Option<u32>,

    pre_up: Option<String>,
    post_up: Option<String>,
    pre_down: Option<String>,
    post_down: Option<String>,
}

impl Interface {
    //noinspection DuplicatedCode
    pub fn tag(&self) -> String {
        if let Some(ref tag) = self.tag {
            return format!("#{}", tag.to_string());
        }
        return String::new();
    }

    pub fn address(&self) -> String {
        if let Some(ref address_list) = self.address {
            return address_list
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        return String::from("empty");
    }

    pub fn private_key(&self) -> String {
        if let Some(ref private_key) = self.private_key {
            return private_key.to_string();
        }
        return String::from("empty");
    }

    pub fn listen_port(&self) -> String {
        if let Some(ref listen_port) = self.listen_port {
            return listen_port.to_string();
        }
        return String::from("empty");
    }

    pub fn mtu(&self) -> String {
        if let Some(ref mtu) = self.mtu {
            return mtu.to_string();
        }
        return String::from("empty");
    }
}

pub struct Peer {
    // tag name
    tag: Option<String>,
    // peer's public key
    public_key: Option<String>,
    // peer's router allowed_ips
    allowed_ips: Option<Vec<ipnet::IpNet>>,
    // keep alive interval
    persistent_keepalive: Option<u16>,
    // peet endpoint
    endpoint: Option<PeerEndpoint>,
}

impl Peer {
    //noinspection DuplicatedCode
    pub fn tag(&self) -> String {
        if let Some(ref tag) = self.tag {
            return format!("#{}", tag.to_string());
        }
        return String::new();
    }

    pub fn allowed_ips(&self) -> String {
        if let Some(ref allowed_ips) = self.allowed_ips {
            return allowed_ips
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        return String::from("empty");
    }

    pub fn public_key(&self) -> String {
        if let Some(ref public_key) = self.public_key {
            return public_key.to_string();
        }
        return String::from("empty");
    }

    pub fn persistent_keepalive(&self) -> String {
        if let Some(ref persistent_keepalive) = self.persistent_keepalive {
            return persistent_keepalive.to_string();
        }
        return String::from("empty");
    }

    pub fn endpoint(&self) -> String {
        if let Some(ref endpoint) = self.endpoint {
            return endpoint.to_string();
        }
        return String::from("empty");
    }
}

pub struct Node {
    // node mode (client/server)
    is_server: bool,
    // peer tag name
    tag: Option<String>,
    // server peer address
    address: Option<Vec<ipnet::IpNet>>,
    // peet's public key
    public_key: Option<String>,
    // peet's private key
    private_key: Option<String>,
    // peet's listen port
    listen_port: Option<String>,
    // peet's router allowed ips
    allowed_ips: Option<Vec<ipnet::IpNet>>,
    // peet's endpoint router allowed ips
    endpoint_allowed_ips: Option<Vec<ipnet::IpNet>>,
    // keep alive interval
    persistent_keepalive: Option<u16>,
    // peet endpoint
    endpoint: Option<PeerEndpoint>,

    pub mtu: Option<u32>,

    pub pre_up: Option<String>,

    pub post_up: Option<String>,

    pub pre_down: Option<String>,

    pub post_down: Option<String>,
}

impl Node {
    pub fn to_interface(self) -> (Interface, Option<String>) {
        (
            Interface {
                is_server: self.is_server,
                tag: self.tag,
                address: self.address,
                private_key: self.private_key,
                listen_port: self.listen_port,
                mtu: self.mtu,
                pre_up: self.pre_up,
                post_down: self.post_down,
                pre_down: self.pre_down,
                post_up: self.post_up,
            },
            self.public_key,
        )
    }

    pub fn to_peer(self) -> (Peer, Option<String>) {
        (
            Peer {
                tag: self.tag,
                public_key: self.public_key,
                allowed_ips: self.allowed_ips,
                persistent_keepalive: self.persistent_keepalive,
                endpoint: self.endpoint,
            },
            self.private_key,
        )
    }
}

pub struct PeerEndpoint {
    address: std::net::IpAddr,
    port: u16,
}

impl PeerEndpoint {
    pub fn new(addr: std::net::IpAddr, port: u16) -> Self {
        Self {
            address: addr,
            port,
        }
    }
}

impl ToString for PeerEndpoint {
    fn to_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
