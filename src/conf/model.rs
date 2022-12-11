use crate::args::{AddPeer, AddServer};
use crate::conf::endpoint::{Endpoint, Interface, IpNet, Peer};
use crate::wg;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::ops::Deref;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WireGuardConfig {
    node_server: Option<Node>,
    node_list: Option<Vec<Node>>,
}

impl WireGuardConfig {
    /*
    set node server
     */
    pub fn node_server(&mut self, node: Node) {
        if let Some(ref mut n) = self.node_server {
            WireGuardConfig::map_set(n, node)
        } else {
            self.node_server = Some(node)
        }
    }

    /*
    push node to list
    */
    pub fn push_node(&mut self, node: Node) {
        let peer_list = self.node_list.get_or_insert_with(Vec::new);
        if let Some(name) = &node.name {
            if let Some(index) = peer_list.iter().position(|n| n.name().eq(name)) {
                WireGuardConfig::map_set(&mut peer_list[index], node);
            } else {
                peer_list.push(node);
            }
        }
    }

    /*
    remove from node list
     */
    pub fn remove_node(&mut self, node_name: String) {
        if let Some(peer_list) = self.node_list.as_mut() {
            if let Some(index) = peer_list.iter().position(|n| n.name().eq(&node_name)) {
                peer_list.remove(index);
            }
        }
    }

    /*
    get from node list
     */
    pub fn get_node_list(&mut self) -> &mut Vec<Node> {
        self.node_list.get_or_insert_with(Vec::new)
    }

    // replace if not present
    fn map_set(change: &mut Node, node: Node) {
        // node name
        change.with_name(node.name);
        // node endpoint(peer)
        if node.endpoint.is_some() {
            change.with_endpoint(node.endpoint);
        }
        // node address(server)
        if node.address.is_some() {
            change.with_address(node.address);
        }
        // node listen port(server)
        if node.listen_port.is_some() {
            change.with_listen_port(node.listen_port);
        }
        // node MTU
        if node.mtu.is_some() {
            change.with_mtu(node.mtu);
        }
        // node allowed ips
        if node.allowed_ips.is_some() {
            change.with_allowed_ips(node.allowed_ips);
        }
        // node endpoint allowed ips(peer)
        if node.endpoint_allowed_ips.is_some() {
            change.with_endpoint_allowed_ips(node.endpoint_allowed_ips);
        }
        // node persistent keepalive
        if node.persistent_keepalive.is_some() {
            change.with_persistent_keepalive(node.persistent_keepalive);
        }
        // If the public key exists it will not be updated
        if node.public_key.is_some() {
            if change.public_key.is_none() {
                change.with_public_key(node.public_key);
            }
        }
        // If the private key exists, it will not be updated
        if node.private_key.is_some() {
            if change.private_key.is_none() {
                change.with_private_key(node.private_key);
            }
        }
        // interface PostUp
        if node.post_up.is_some() {
            change.with_post_up(node.post_up);
        }
        // interface PostDown
        if node.post_down.is_some() {
            change.with_post_down(node.post_down);
        }
        // interface PreUp
        if node.pre_up.is_some() {
            change.with_pre_up(node.pre_up);
        }
        // interface PreDown
        if node.pre_down.is_some() {
            change.with_pre_down(node.pre_down);
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

// node configuration of wireguard
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Node {
    // node name
    name: Option<String>,
    // server node address
    address: Option<Vec<IpNet>>,
    // node's public key
    public_key: Option<String>,
    // node's private key
    private_key: Option<String>,
    // node's listen port
    listen_port: Option<u16>,
    // node's router allowed ips
    allowed_ips: Option<Vec<IpNet>>,
    // node's endpoint router allowed ips
    endpoint_allowed_ips: Option<Vec<IpNet>>,
    // node's keep alive interval
    persistent_keepalive: Option<u16>,
    // node's peer endpoint
    endpoint: Option<Endpoint>,
    // node's MTU
    mtu: Option<u16>,
    // node's PreUP
    pre_up: Option<String>,
    // node's PostUp
    post_up: Option<String>,
    // node's PreDown
    pre_down: Option<String>,
    // node's PostDown
    post_down: Option<String>,
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

// impl Into<Interface> for Node {
//     fn into(self) -> Interface {
//         let mut interface = Interface::default();
//         interface.with_address(self.address)
//             .with_listen_port(self.listen_port)
//             .with_mtu(self.mtu)
//             .with_private_key(self.private_key)
//             .with_post_up(self.post_up)
//             .with_post_down(self.post_down)
//             .with_pre_up(self.pre_up)
//             .with_pre_down(self.pre_down);
//         interface
//     }
// }
//
// impl Into<Peer> for Node {
//     fn into(self) -> Peer {
//         let mut peer = Peer::default();
//         peer.with_mtu(self.mtu)
//             .with_public_key(self.public_key)
//             .with_persistent_keepalive(self.persistent_keepalive)
//             .with_endpoint(self.endpoint)
//             .with_allowed_ips(self.allowed_ips);
//         peer
//     }
// }

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
