use crate::conf::endpoint::Node;
use std::ops::Deref;

use crate::conf::NodeOpt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct WireGuard {
    node_server: Option<Node>,
    node_list: Option<Vec<Node>>,
}

impl WireGuard {
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
}

#[async_trait::async_trait]
impl NodeOpt for WireGuard {
    async fn get_relay(&mut self) -> anyhow::Result<Node> {
        return match &self.node_server {
            None => Err(anyhow::anyhow!("node server does not exits")),
            Some(node) => Ok(node.deref().clone()),
        };
    }

    async fn get_by_name(&mut self, node_name: &str) -> anyhow::Result<Node> {
        let node_list = self.node_list.get_or_insert_with(Vec::new);
        if let Some(index) = node_list.iter().position(|n| n.name().eq(node_name)) {
            let node = node_list.get(index).unwrap();
            return Ok(node.clone());
        }
        Err(anyhow::anyhow!(format!(
            "node does not exist: {}",
            node_name
        )))
    }

    async fn set_relay(&mut self, node: Node) -> anyhow::Result<()> {
        if let Some(ref mut n) = self.node_server {
            Self::map_set(n, node)
        } else {
            self.node_server = Some(node)
        }
        Ok(())
    }

    async fn push(&mut self, node: Node) -> anyhow::Result<()> {
        if self.node_server.is_none() {
            return Err(anyhow::anyhow!("please add Server Peer Node first"));
        }
        let node_list = self.node_list.get_or_insert_with(Vec::new);
        if let Some(name) = &node.name {
            if let Some(index) = node_list.iter().position(|n| n.name().eq(name)) {
                Self::map_set(&mut node_list[index], node);
            } else {
                node_list.push(node);
            }
        }
        Ok(())
    }

    async fn list(&mut self) -> anyhow::Result<Vec<Node>> {
        Ok(self.node_list.get_or_insert_with(Vec::new).clone())
    }

    async fn clear(&mut self) -> anyhow::Result<()> {
        self.drop().await
    }

    async fn remove_all(&mut self) -> anyhow::Result<()> {
        if let Some(node_list) = self.node_list.as_mut() {
            node_list.clear();
        }
        Ok(())
    }

    async fn remove_by_name(&mut self, node_name: &str) -> anyhow::Result<()> {
        if let Some(node_list) = self.node_list.as_mut() {
            if let Some(index) = node_list.iter().position(|n| n.name().eq(node_name)) {
                node_list.remove(index);
                return Ok(());
            }
        }
        Err(anyhow::anyhow!(format!(
            "there is no node named '{}'",
            node_name
        )))
    }

    async fn remove_by_index(&mut self, index: usize) -> anyhow::Result<()> {
        if let Some(node_list) = self.node_list.as_mut() {
            if index >= node_list.len() {
                return Err(anyhow::anyhow!(format!(
                    "index data {} out of bounds",
                    index
                )));
            }
            node_list.remove(index);
        }
        Ok(())
    }

    async fn drop(&mut self) -> anyhow::Result<()> {
        self.node_server = None;
        self.node_list = None;
        Ok(())
    }

    async fn exist(&self, name: String) -> bool {
        if let Some(node_list) = self.node_list.as_ref() {
            return node_list
                .iter()
                .map(|x| x.name())
                .collect::<Vec<&str>>()
                .contains(&name.as_str());
        }
        false
    }
}
