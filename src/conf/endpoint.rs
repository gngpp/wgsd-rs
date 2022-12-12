use crate::args::{AddPeer, AddServer};
use crate::wg;
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
    pub fn address(&self) -> String {
        if let Some(ref address_list) = self.address {
            return address_list
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",");
        }
        String::new()
    }

    pub fn private_key(&self) -> &str {
        self.private_key.as_deref().unwrap_or_default()
    }

    pub fn listen_port(&self) -> &str {
        self.post_down.as_deref().unwrap_or_default()
    }

    pub fn mtu(&self) -> Option<u16> {
        self.mtu.clone()
    }

    pub fn post_up(&self) -> Option<&str> {
        self.post_down.as_deref()
    }

    pub fn post_down(&self) -> Option<&str> {
        self.post_down.as_deref()
    }

    pub fn pre_up(&self) -> Option<&str> {
        self.post_down.as_deref()
    }

    pub fn pre_down(&self) -> Option<String> {
        if let Some(ref pre_down) = self.pre_down {
            return Some(pre_down.to_string());
        }
        None
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
    pub fn allowed_ips(&self) -> String {
        if let Some(ref allowed_ips) = self.allowed_ips {
            return allowed_ips
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ");
        }
        return String::new();
    }

    pub fn public_key(&self) -> String {
        if let Some(ref public_key) = self.public_key {
            return public_key.to_string();
        }
        return String::new();
    }

    pub fn persistent_keepalive(&self) -> Option<&u16> {
        self.persistent_keepalive.as_ref()
    }

    pub fn endpoint(&self) -> String {
        if let Some(ref endpoint) = self.endpoint {
            return endpoint.to_string();
        }
        return String::new();
    }

    pub fn mtu(&self) -> u16 {
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

// node configuration of wireguard
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Node {
    // node name
    pub name: Option<String>,
    // server node address
    pub address: Option<Vec<IpNet>>,
    // node's public key
    pub public_key: Option<String>,
    // node's private key
    pub private_key: Option<String>,
    // node's listen port
    pub listen_port: Option<u16>,
    // node's router allowed ips
    pub allowed_ips: Option<Vec<IpNet>>,
    // node's endpoint router allowed ips
    pub endpoint_allowed_ips: Option<Vec<IpNet>>,
    // node's keep alive interval
    pub persistent_keepalive: Option<u16>,
    // node's peer endpoint
    pub endpoint: Option<Endpoint>,
    // node's MTU
    pub mtu: Option<u16>,
    // node's PreUP
    pub pre_up: Option<String>,
    // node's PostUp
    pub post_up: Option<String>,
    // node's PreDown
    pub pre_down: Option<String>,
    // node's PostDown
    pub post_down: Option<String>,
}

impl Node {
    /*
     * Builder functions
     */

    pub fn with_name(&mut self, name: Option<String>) -> &mut Node {
        self.name = name;
        self
    }
    pub fn with_address(&mut self, address: Option<Vec<IpNet>>) -> &mut Node {
        self.address = address;
        self
    }
    pub fn with_public_key(&mut self, public_key: Option<String>) -> &mut Node {
        self.public_key = public_key;
        self
    }
    pub fn with_private_key(&mut self, private_key: Option<String>) -> &mut Node {
        self.private_key = private_key;
        self
    }
    pub fn with_listen_port(&mut self, listen_port: Option<u16>) -> &mut Node {
        self.listen_port = listen_port;
        self
    }
    pub fn with_allowed_ips(&mut self, allowed_ips: Option<Vec<IpNet>>) -> &mut Node {
        self.allowed_ips = allowed_ips;
        self
    }
    pub fn with_endpoint_allowed_ips(
        &mut self,
        endpoint_allowed_ips: Option<Vec<IpNet>>,
    ) -> &mut Node {
        self.endpoint_allowed_ips = endpoint_allowed_ips;
        self
    }
    pub fn with_persistent_keepalive(&mut self, persistent_keepalive: Option<u16>) -> &mut Node {
        self.persistent_keepalive = persistent_keepalive;
        self
    }
    pub fn with_endpoint(&mut self, endpoint: Option<Endpoint>) -> &mut Node {
        self.endpoint = endpoint;
        self
    }
    pub fn with_mtu(&mut self, mtu: Option<u16>) -> &mut Node {
        self.mtu = mtu;
        self
    }
    pub fn with_pre_up(&mut self, pre_up: Option<String>) -> &mut Node {
        self.pre_up = pre_up;
        self
    }
    pub fn with_post_up(&mut self, post_up: Option<String>) -> &mut Node {
        self.post_up = post_up;
        self
    }
    pub fn with_pre_down(&mut self, pre_down: Option<String>) -> &mut Node {
        self.pre_down = pre_down;
        self
    }
    pub fn with_post_down(&mut self, post_down: Option<String>) -> &mut Node {
        self.post_down = post_down;
        self
    }
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or_default()
    }
}

impl Into<Interface> for Node {
    fn into(self) -> Interface {
        let mut interface = Interface::default();
        interface
            .with_private_key(self.private_key)
            .with_address(self.address)
            .with_listen_port(self.listen_port)
            .with_mtu(self.mtu)
            .with_post_up(self.post_up)
            .with_post_down(self.post_down)
            .with_pre_up(self.pre_up)
            .with_pre_down(self.pre_down);
        interface
    }
}

impl Into<Peer> for Node {
    fn into(self) -> Peer {
        let mut peer = Peer::default();
        let addrs = self.address.unwrap_or_default();
        let mut allowed_ips = self.allowed_ips.unwrap_or_default();
        allowed_ips.extend(addrs);
        peer.with_public_key(self.public_key)
            .with_persistent_keepalive(self.persistent_keepalive)
            .with_allowed_ips(Some(allowed_ips))
            .with_mtu(self.mtu)
            .with_endpoint(self.endpoint);
        peer
    }
}

impl From<AddServer> for Node {
    fn from(add_server: AddServer) -> Self {
        let mut node = Node::default();
        let key_pair = wg::WireGuardCommand::generate_key_pair(false).unwrap();
        node.with_name(Some(add_server.name))
            .with_endpoint(Some(Endpoint::new(
                add_server.endpoint,
                add_server.listen_port,
            )))
            .with_address(Some(add_server.address))
            .with_listen_port(Some(add_server.listen_port))
            .with_mtu(Some(add_server.mtu))
            .with_public_key(Some(key_pair.public_key().to_string()))
            .with_private_key(Some(key_pair.private_key().to_string()))
            .with_post_up(add_server.post_up)
            .with_post_down(add_server.post_down)
            .with_pre_up(add_server.pre_up)
            .with_pre_down(add_server.pre_down);
        node
    }
}

impl From<AddPeer> for Node {
    fn from(add_peer: AddPeer) -> Self {
        let mut node = Node::default();
        let key_pair = wg::WireGuardCommand::generate_key_pair(false).unwrap();
        node.with_name(Some(add_peer.name))
            .with_address(Some(add_peer.address))
            .with_allowed_ips(Some(add_peer.allowed_ips))
            .with_endpoint_allowed_ips(Some(add_peer.endpoint_allowed_ips))
            .with_mtu(Some(add_peer.mtu))
            .with_persistent_keepalive(Some(add_peer.persistent_keepalive))
            .with_public_key(Some(key_pair.public_key().to_string()))
            .with_private_key(Some(key_pair.private_key().to_string()))
            .with_post_up(add_peer.post_up)
            .with_post_down(add_peer.post_down)
            .with_pre_up(add_peer.pre_up)
            .with_pre_down(add_peer.pre_down);
        node
    }
}