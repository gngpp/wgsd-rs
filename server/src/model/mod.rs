use ipnet::IpNet;
use crate::args::{AddPeer, NewPeerRelayNetwork};
use crate::model::endpoint::Endpoint;
use serde::{Deserialize, Serialize};
use crate::wg;

pub mod endpoint;
pub mod node;
pub mod prelude;

// node configuration of wireguard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    // relay node
    pub relay: bool,
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

    pub fn with_relay(&mut self, relay: bool) -> &mut Node {
        self.relay = relay;
        self
    }
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
        self.name.as_deref().expect("peer is not named")
    }
}

impl From<NewPeerRelayNetwork> for Node {
    fn from(add_peer_relay: NewPeerRelayNetwork) -> Self {
        let mut node = Node::default();
        let key_pair = wg::WireGuardCommand::generate_key_pair(false).unwrap();
        node.with_relay(true)
            .with_name(Some(add_peer_relay.name))
            .with_endpoint(Some(Endpoint::new(
                add_peer_relay.endpoint,
                add_peer_relay.listen_port,
            )))
            .with_address(Some(add_peer_relay.address))
            .with_listen_port(Some(add_peer_relay.listen_port))
            .with_mtu(Some(add_peer_relay.mtu))
            .with_public_key(Some(key_pair.public_key().to_string()))
            .with_private_key(Some(key_pair.private_key().to_string()))
            .with_post_up(add_peer_relay.post_up)
            .with_post_down(add_peer_relay.post_down)
            .with_pre_up(add_peer_relay.pre_up)
            .with_pre_down(add_peer_relay.pre_down);
        node
    }
}

impl From<AddPeer> for Node {
    fn from(add_peer: AddPeer) -> Self {
        let mut node = Node::default();
        let key_pair = wg::WireGuardCommand::generate_key_pair(false).unwrap();
        node.with_relay(false)
            .with_name(Some(add_peer.name))
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

