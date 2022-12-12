use std::ops::Not;

use crate::conf::endpoint::{Interface, Node, Peer};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WireGuard {
    node_server: Option<Node>,
    node_list: Option<Vec<Node>>,
}

impl WireGuard {
    // set node server
    pub fn set(&mut self, node: Node) -> anyhow::Result<()> {
        if let Some(ref mut n) = self.node_server {
            WireGuard::map_set(n, node)
        } else {
            self.node_server = Some(node)
        }
        Ok(())
    }

    // push node to list
    pub fn push(&mut self, node: Node) -> anyhow::Result<()> {
        if self.node_server.is_none() {
            anyhow::bail!("Please add Server Peer Node first");
        }
        let peer_list = self.node_list.get_or_insert_with(Vec::new);
        if let Some(name) = &node.name {
            if let Some(index) = peer_list.iter().position(|n| n.name().eq(name)) {
                WireGuard::map_set(&mut peer_list[index], node);
            } else {
                peer_list.push(node)
            }
        }
        Ok(())
    }

    // remove from node list
    pub fn remove(&mut self, node_name: String) -> anyhow::Result<()> {
        if let Some(peer_list) = self.node_list.as_mut() {
            if let Some(index) = peer_list.iter().position(|n| n.name().eq(&node_name)) {
                peer_list.remove(index);
            }
        }
        Ok(())
    }

    // get from node list
    pub fn list(&mut self) -> anyhow::Result<&mut Vec<Node>> {
        Ok(self.node_list.get_or_insert_with(Vec::new))
    }

    // exist peer
    pub fn exist(&self, name: String) -> bool {
        if let Some(peer_list) = self.node_list.as_ref() {
            return peer_list
                .iter()
                .map(|x| x.name())
                .collect::<Vec<&str>>()
                .contains(&name.as_str());
        }
        false
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

    pub fn to_server_configuration_str(&self) -> anyhow::Result<Option<String>> {
        if let Some(node_server) = self.node_server.clone() {
            let mut lines = String::new();
            // node name
            lines.push_str(&format!("# {}\n", node_server.name()));

            let interface = Interface::from(node_server);

            // Interface section begins
            lines.push_str("[Interface]\n");

            // Interface Private key
            lines.push_str(&format!("PrivateKey = {}\n", interface.private_key()?));

            // Interface address
            lines.push_str(&format!("Address = {}\n", interface.address()?));

            // Interface listen port
            lines.push_str(&format!("ListenPort = {}\n", interface.listen_port()?));

            // MTU, if any
            if let Some(mtu) = interface.mtu() {
                lines.push_str(&format!("MTU = {}\n", mtu));
            }

            // PreUp, if any
            if let Some(pre_up) = interface.pre_up() {
                lines.push_str(&format!("PreUp = {}\n", pre_up));
            }

            // PostUp, if any
            if let Some(post_up) = interface.post_up() {
                lines.push_str(&format!("PostUp = {}\n", post_up));
            }

            // PreDown, if any
            if let Some(pre_down) = interface.pre_down() {
                lines.push_str(&format!("PreDown = {}\n", pre_down));
            }

            // PostDown, if any
            if let Some(post_down) = interface.post_down() {
                lines.push_str(&format!("PostDown = {}\n", post_down));
            }

            if let Some(node_list) = self.node_list.clone() {
                for node_peer in node_list {
                    // node name
                    lines.push_str(&format!("# {}\n", node_peer.name()));

                    let peer = Peer::from(node_peer);

                    // Peer section begins
                    lines.push_str("[Peer]\n");

                    // Peer Public key
                    lines.push_str(&format!("PublicKey = {}\n", peer.public_key()?));

                    // Peer Allowed IPs
                    lines.push_str(&format!("AllowedIPs = {}\n", peer.allowed_ips()?));

                    // Keepalive
                    if let Some(keepalive) = peer.persistent_keepalive() {
                        lines.push_str(&format!("PersistentKeepalive = {}\n", keepalive));
                    }
                }
            }
            return Ok(Some(lines));
        }
        Ok(None)
    }

    pub fn to_peer_configuration_str(&self) -> Option<String> {
        if let Some(node_list) = self.node_list.clone() {
            if node_list.is_empty().not() {
                println!("{:?}", node_list);
            }
        }
        None
    }
}
