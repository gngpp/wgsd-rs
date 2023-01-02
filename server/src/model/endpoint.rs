use crate::args::{AddPeer, NewPeerRelayNetwork};
use crate::wg;
use anyhow::Context;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};
use crate::model::Node;

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
    pub fn address(&self) -> anyhow::Result<String> {
        Ok(self
            .address
            .as_deref()
            .context("address is undefined")?
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", "))
    }

    pub fn private_key(&self) -> anyhow::Result<&str> {
        self.private_key
            .as_deref()
            .context("private key is undefined")
    }

    pub fn listen_port(&self) -> Option<u16> {
        self.listen_port
    }

    pub fn mtu(&self) -> Option<&u16> {
        self.mtu.as_ref()
    }

    pub fn post_up(&self) -> Option<&str> {
        self.post_up.as_deref()
    }

    pub fn post_down(&self) -> Option<&str> {
        self.post_down.as_deref()
    }

    pub fn pre_up(&self) -> Option<&str> {
        self.pre_up.as_deref()
    }

    pub fn pre_down(&self) -> Option<&str> {
        self.pre_down.as_deref()
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

impl From<Node> for Interface {
    fn from(node: Node) -> Self {
        let mut interface = Interface::default();
        interface
            .with_private_key(node.private_key)
            .with_address(node.address)
            .with_listen_port(node.listen_port)
            .with_mtu(node.mtu)
            .with_post_up(node.post_up)
            .with_post_down(node.post_down)
            .with_pre_up(node.pre_up)
            .with_pre_down(node.pre_down);
        interface
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
    pub fn allowed_ips(&self) -> anyhow::Result<String> {
        Ok(self
            .allowed_ips
            .as_deref()
            .context("allowed_ips is undefined")?
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(", "))
    }

    pub fn public_key(&self) -> anyhow::Result<&str> {
        self.public_key
            .as_deref()
            .context("public key is undefined")
    }

    pub fn endpoint(&self) -> Option<String> {
        if let Some(ref endpoint) = self.endpoint {
            return Some(endpoint.to_string());
        }
        None
    }

    pub fn persistent_keepalive(&self) -> Option<&u16> {
        self.persistent_keepalive.as_ref()
    }

    pub fn mtu(&self) -> Option<&u16> {
        self.mtu.as_ref()
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

impl From<Node> for Peer {
    fn from(node: Node) -> Self {
        let mut peer = Peer::default();
        if node.relay {
            peer.with_public_key(node.public_key)
                .with_persistent_keepalive(node.persistent_keepalive)
                // peer relay allowed_ips
                .with_allowed_ips(node.allowed_ips)
                .with_mtu(node.mtu)
                .with_endpoint(node.endpoint);
        } else {
            // peer relay address: 10.6.0.1/24
            // examples:
            let mut allowed_ips = node.allowed_ips.unwrap_or_default();
            allowed_ips.extend(node.address.unwrap_or_default());
            peer.with_public_key(node.public_key)
                .with_persistent_keepalive(node.persistent_keepalive)
                // peer relay allowed_ips
                .with_allowed_ips(Some(allowed_ips))
                .with_mtu(node.mtu)
                .with_endpoint(node.endpoint);
        }

        peer
    }
}

// peer endpoint configuration of wireguard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Endpoint {
    address: String,
    port: u16,
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