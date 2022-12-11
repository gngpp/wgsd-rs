use crate::conf::model::Node;
use serde::{Deserialize, Serialize};

// interface configuration of wireguard
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Interface {
    // interface's WireGuard address
    address: Option<Vec<IpNet>>,
    // interface's private key
    private_key: Option<String>,
    // interface's listen port
    listen_port: Option<u16>,
    // interface's MTU
    mtu: Option<u16>,
    // interface's PreUP
    pre_up: Option<String>,
    // interface's PostUp
    post_up: Option<String>,
    // interface's PreDown
    pre_down: Option<String>,
    // interface's PostDown
    post_down: Option<String>,
}

impl Interface {
    fn get_mtu(&self) -> u16 {
        self.mtu.unwrap_or_default()
    }

    pub fn get_address(&self) -> String {
        if let Some(ref address_list) = self.address {
            return address_list
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        return String::new();
    }

    pub fn get_private_key(&self) -> String {
        if let Some(ref private_key) = self.private_key {
            return private_key.to_string();
        }
        return String::new();
    }

    pub fn get_listen_port(&self) -> String {
        if let Some(ref listen_port) = self.listen_port {
            return listen_port.to_string();
        }
        return String::new();
    }

    pub fn get_post_up(&self) -> String {
        if let Some(ref post_up) = self.post_up {
            return post_up.to_string();
        }
        return String::new();
    }

    pub fn get_post_down(&self) -> String {
        if let Some(ref post_down) = self.post_down {
            return post_down.to_string();
        }
        return String::new();
    }

    pub fn get_pre_up(&self) -> String {
        if let Some(ref post_up) = self.post_up {
            return post_up.to_string();
        }
        return String::new();
    }

    pub fn get_pre_down(&self) -> String {
        if let Some(ref pre_down) = self.pre_down {
            return pre_down.to_string();
        }
        return String::new();
    }

    pub fn with_address(&mut self, address: Option<Vec<IpNet>>) -> &mut Interface {
        self.address = address;
        self
    }
    pub fn with_private_key(&mut self, private_key: Option<String>) -> &mut Interface {
        self.private_key = private_key;
        self
    }
    pub fn with_listen_port(&mut self, listen_port: Option<u16>) -> &mut Interface {
        self.listen_port = listen_port;
        self
    }
    pub fn with_mtu(&mut self, mtu: Option<u16>) -> &mut Interface {
        self.mtu = mtu;
        self
    }
    pub fn with_pre_up(&mut self, pre_up: Option<String>) -> &mut Interface {
        self.pre_up = pre_up;
        self
    }
    pub fn with_post_up(&mut self, post_up: Option<String>) -> &mut Interface {
        self.post_up = post_up;
        self
    }
    pub fn with_pre_down(&mut self, pre_down: Option<String>) -> &mut Interface {
        self.pre_down = pre_down;
        self
    }
    pub fn with_post_down(&mut self, post_down: Option<String>) -> &mut Interface {
        self.post_down = post_down;
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Peer {
    // peer's public key
    public_key: Option<String>,
    // peer's router allowed_ips
    allowed_ips: Option<Vec<IpNet>>,
    // peer's keep alive interval
    persistent_keepalive: Option<u16>,
    // peer's MTU
    mtu: Option<u16>,
    // peer's endpoint
    endpoint: Option<Endpoint>,
}

// peer configuration of wireguard
impl Peer {
    pub fn get_allowed_ips(&self) -> String {
        if let Some(ref allowed_ips) = self.allowed_ips {
            return allowed_ips
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        return String::new();
    }

    pub fn get_public_key(&self) -> String {
        if let Some(ref public_key) = self.public_key {
            return public_key.to_string();
        }
        return String::new();
    }

    pub fn get_persistent_keepalive(&self) -> String {
        if let Some(ref persistent_keepalive) = self.persistent_keepalive {
            return persistent_keepalive.to_string();
        }
        return String::new();
    }

    pub fn get_endpoint(&self) -> String {
        if let Some(ref endpoint) = self.endpoint {
            return endpoint.to_string();
        }
        return String::new();
    }

    fn get_mtu(&self) -> u16 {
        self.mtu.unwrap_or_default()
    }

    pub fn with_public_key(&mut self, public_key: Option<String>) -> &mut Peer {
        self.public_key = public_key;
        self
    }
    pub fn with_allowed_ips(&mut self, allowed_ips: Option<Vec<IpNet>>) -> &mut Peer {
        self.allowed_ips = allowed_ips;
        self
    }
    pub fn with_persistent_keepalive(&mut self, persistent_keepalive: Option<u16>) -> &mut Peer {
        self.persistent_keepalive = persistent_keepalive;
        self
    }
    pub fn with_mtu(&mut self, mtu: Option<u16>) -> &mut Peer {
        self.mtu = mtu;
        self
    }
    pub fn with_endpoint(&mut self, endpoint: Option<Endpoint>) -> &mut Peer {
        self.endpoint = endpoint;
        self
    }
}

// peer endpoint configuration of wireguard
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub address: String,
    pub port: u16,
}

impl Endpoint {
    pub fn new(addr: String, port: u16) -> Self {
        Self {
            address: addr,
            port,
        }
    }
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IpNet {
    pub value: String,
}

impl From<ipnet::IpNet> for IpNet {
    fn from(i: ipnet::IpNet) -> Self {
        IpNet {
            value: i.to_string(),
        }
    }
}

impl ToString for IpNet {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
