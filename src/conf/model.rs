use super::NodeProperty;
use serde::{Deserialize, Serialize};

extern crate ipnet;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WG {
    server_peer_node: Node,
    peer_node_list: Vec<Node>,
}

impl WG {
    pub fn new(server_peer_node: Node, peer_node_list: Vec<Node>) -> WG {
        WG {
            server_peer_node,
            peer_node_list,
        }
    }
    // pub fn get_config(&self) -> Vec<String> {
    //     let mut lines: Vec<String> = Vec::new();
    //     lines.push(String::from("[Interface]"));
    //     lines.push(format!("Address = {}", self.server_peer_node.get_address()));
    //     lines.push(format!("PrivateKey = {}", self.server_peer_node.get_private_key()));
    //     lines.push(format!("ListenPort = {}", self.server_peer_node.get_listen_port()));
    //     lines.push(format!("MTU = {}\n", self.server_peer_node.get_mtu()));
    //     lines.push(format!(
    //         "Description = {}",
    //         self.server_peer_node.get_description()
    //     ));
    //
    //     for peer in self.peer_node_list.iter() {
    //         lines.push(String::from("[Peer]"));
    //         lines.push(format!("PublicKey = {}", peer.get_public_key()));
    //         lines.push(format!("AllowedIPs = {}", peer.get_allowed_ips()));
    //         lines.push(format!("Endpoint = {}", peer.get_endpoint()));
    //         lines.push(format!(
    //             "PersistentKeepalive = {}\n",
    //             peer.get_persistent_keepalive()
    //         ));
    //         lines.push(format!("Description = {}", peer.get_description()))
    //     }
    //     lines
    // }
}

// interface configuration of wireguard
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Interface {
    // description
    description: Option<String>,
    // interface WireGuard address
    address: Option<Vec<IpNet>>,
    // interface private key
    private_key: Option<String>,
    // interface listen port
    listen_port: Option<u16>,
    // interface MTU
    mtu: Option<u16>,

    pre_up: Option<String>,
    post_up: Option<String>,
    pre_down: Option<String>,
    post_down: Option<String>,
}

impl Interface {
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

    pub fn set_description(&mut self, description: Option<String>) -> &mut Interface {
        self.description = description;
        self
    }
    pub fn set_address(&mut self, address: Option<Vec<IpNet>>) -> &mut Interface {
        self.address = address;
        self
    }
    pub fn set_private_key(&mut self, private_key: Option<String>) -> &mut Interface {
        self.private_key = private_key;
        self
    }
    pub fn set_listen_port(&mut self, listen_port: Option<u16>) -> &mut Interface {
        self.listen_port = listen_port;
        self
    }
    pub fn set_mtu(&mut self, mtu: Option<u16>) -> &mut Interface {
        self.mtu = mtu;
        self
    }
    pub fn set_pre_up(&mut self, pre_up: Option<String>) -> &mut Interface {
        self.pre_up = pre_up;
        self
    }
    pub fn set_post_up(&mut self, post_up: Option<String>) -> &mut Interface {
        self.post_up = post_up;
        self
    }
    pub fn set_pre_down(&mut self, pre_down: Option<String>) -> &mut Interface {
        self.pre_down = pre_down;
        self
    }
    pub fn set_post_down(&mut self, post_down: Option<String>) -> &mut Interface {
        self.post_down = post_down;
        self
    }
}

impl NodeProperty for Interface {
    fn get_description(&self) -> String {
        if let Some(ref description) = self.description {
            return description.to_string();
        }
        return String::new();
    }

    fn get_mtu(&self) -> u16 {
        self.mtu.unwrap_or_default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Peer {
    // description
    description: Option<String>,
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
    pub fn set_description(&mut self, description: Option<String>) -> &mut Peer {
        self.description = description;
        self
    }
    pub fn set_public_key(&mut self, public_key: Option<String>) -> &mut Peer {
        self.public_key = public_key;
        self
    }
    pub fn set_allowed_ips(&mut self, allowed_ips: Option<Vec<IpNet>>) -> &mut Peer {
        self.allowed_ips = allowed_ips;
        self
    }
    pub fn set_persistent_keepalive(&mut self, persistent_keepalive: Option<u16>) -> &mut Peer {
        self.persistent_keepalive = persistent_keepalive;
        self
    }
    pub fn set_mtu(&mut self, mtu: Option<u16>) -> &mut Peer {
        self.mtu = mtu;
        self
    }
    pub fn set_endpoint(&mut self, endpoint: Option<Endpoint>) -> &mut Peer {
        self.endpoint = endpoint;
        self
    }
}

impl NodeProperty for Peer {
    fn get_description(&self) -> String {
        if let Some(ref description) = self.description {
            return description.to_string();
        }
        return String::new();
    }

    fn get_mtu(&self) -> u16 {
        self.mtu.unwrap_or_default()
    }
}

// node configuration of wireguard
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Node {
    is_server: bool,
    // peer tag name
    description: Option<String>,
    // server peer address
    address: Option<Vec<IpNet>>,
    // peet's public key
    public_key: Option<String>,
    // peet's private key
    private_key: Option<String>,
    // peet's listen port
    listen_port: Option<u16>,
    // peet's router allowed ips
    allowed_ips: Option<Vec<IpNet>>,
    // peet's endpoint router allowed ips
    endpoint_allowed_ips: Option<Vec<IpNet>>,
    // peet's keep alive interval
    persistent_keepalive: Option<u16>,
    // peet's peer endpoint
    endpoint: Option<Endpoint>,
    // peet's MTU
    mtu: Option<u16>,

    pre_up: Option<String>,

    post_up: Option<String>,

    pre_down: Option<String>,

    post_down: Option<String>,
}

impl Node {
    pub fn to_interface(self) -> (Interface, Option<String>) {
        (
            Interface {
                description: self.description,
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
                description: self.description,
                public_key: self.public_key,
                allowed_ips: self.allowed_ips,
                persistent_keepalive: self.persistent_keepalive,
                mtu: self.mtu,
                endpoint: self.endpoint,
            },
            self.private_key,
        )
    }

    pub fn set_is_server(&mut self, is_server: bool) -> &mut Node {
        self.is_server = is_server;
        self
    }
    pub fn set_description(&mut self, description: Option<String>) -> &mut Node {
        self.description = description;
        self
    }
    pub fn set_address(&mut self, address: Option<Vec<IpNet>>) -> &mut Node {
        self.address = address;
        self
    }
    pub fn set_public_key(&mut self, public_key: Option<String>) -> &mut Node {
        self.public_key = public_key;
        self
    }
    pub fn set_private_key(&mut self, private_key: Option<String>) -> &mut Node {
        self.private_key = private_key;
        self
    }
    pub fn set_listen_port(&mut self, listen_port: Option<u16>) -> &mut Node {
        self.listen_port = listen_port;
        self
    }
    pub fn set_allowed_ips(&mut self, allowed_ips: Option<Vec<IpNet>>) -> &mut Node {
        self.allowed_ips = allowed_ips;
        self
    }
    pub fn set_endpoint_allowed_ips(
        &mut self,
        endpoint_allowed_ips: Option<Vec<IpNet>>,
    ) -> &mut Node {
        self.endpoint_allowed_ips = endpoint_allowed_ips;
        self
    }
    pub fn set_persistent_keepalive(&mut self, persistent_keepalive: Option<u16>) -> &mut Node {
        self.persistent_keepalive = persistent_keepalive;
        self
    }
    pub fn set_endpoint(&mut self, endpoint: Option<Endpoint>) -> &mut Node {
        self.endpoint = endpoint;
        self
    }
    pub fn set_mtu(&mut self, mtu: Option<u16>) -> &mut Node {
        self.mtu = mtu;
        self
    }
    pub fn set_pre_up(&mut self, pre_up: Option<String>) -> &mut Node {
        self.pre_up = pre_up;
        self
    }
    pub fn set_post_up(&mut self, post_up: Option<String>) -> &mut Node {
        self.post_up = post_up;
        self
    }
    pub fn set_pre_down(&mut self, pre_down: Option<String>) -> &mut Node {
        self.pre_down = pre_down;
        self
    }
    pub fn set_post_down(&mut self, post_down: Option<String>) -> &mut Node {
        self.post_down = post_down;
        self
    }
    pub fn is_server(&self) -> bool {
        self.is_server
    }
    pub fn description(&self) -> String {
        if let Some(ref description) = self.description {
            return description.to_string()
        }
        String::new()
    }
    pub fn address(&self) -> &Option<Vec<IpNet>> {
        &self.address
    }
    pub fn public_key(&self) -> &Option<String> {
        &self.public_key
    }
    pub fn private_key(&self) -> &Option<String> {
        &self.private_key
    }
    pub fn listen_port(&self) -> Option<u16> {
        self.listen_port
    }
    pub fn allowed_ips(&self) -> &Option<Vec<IpNet>> {
        &self.allowed_ips
    }
    pub fn endpoint_allowed_ips(&self) -> &Option<Vec<IpNet>> {
        &self.endpoint_allowed_ips
    }
    pub fn persistent_keepalive(&self) -> Option<u16> {
        self.persistent_keepalive
    }
    pub fn endpoint(&self) -> &Option<Endpoint> {
        &self.endpoint
    }
    pub fn mtu(&self) -> Option<u16> {
        self.mtu
    }
    pub fn pre_up(&self) -> &Option<String> {
        &self.pre_up
    }
    pub fn post_up(&self) -> &Option<String> {
        &self.post_up
    }
    pub fn pre_down(&self) -> &Option<String> {
        &self.pre_down
    }
    pub fn post_down(&self) -> &Option<String> {
        &self.post_down
    }
}

// peer endpoint configuration of wireguard
#[derive(Debug, Default, Serialize, Deserialize)]
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IpNet {
    value: String
}

impl IpNet {
    pub fn new(i: ipnet::IpNet) -> Self {
        Self { value: i.to_string() }
    }
}

impl ToString for IpNet {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}