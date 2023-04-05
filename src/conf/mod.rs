extern crate ipnet;

use anyhow::{anyhow, Context};
use async_trait::async_trait;

use crate::conf::models::WireGuard;
use crate::model::endpoint::{Interface, Peer};
use crate::model::Node;
use std::ops::{DerefMut, Not};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod models;

#[async_trait]
pub trait NodeOpt: Sized {
    // get node by name
    async fn get_by_name(&mut self, node_name: &str) -> anyhow::Result<Node>;
    // push node to list
    async fn push(&mut self, node: Node) -> anyhow::Result<()>;
    // get from node list(by relay)
    async fn list_by_relay(&mut self, relay: bool) -> anyhow::Result<Vec<Node>>;
    // get from node list
    async fn list(&mut self) -> anyhow::Result<Vec<Node>>;
    // remove all from node list
    async fn remove_all(&mut self) -> anyhow::Result<()>;
    // remove node from list
    async fn remove_by_name(&mut self, node_name: &str) -> anyhow::Result<()>;
    // remove node from list
    async fn remove(&mut self, index: usize) -> anyhow::Result<()>;
    // clear
    async fn clear(&mut self) -> anyhow::Result<()>;
}

#[async_trait]
pub trait AsyncTryFrom<T>: Sized {
    type Error;

    async fn try_from(_: T) -> Result<Self, Self::Error>;
}

pub struct Configuration {
    path: PathBuf,
    wireguard: Arc<Mutex<WireGuard>>,
}

impl Configuration {
    async fn init(config_path: Option<PathBuf>) -> anyhow::Result<()> {
        if let Some(config_path) = config_path {
            if !config_path.exists() {
                return Err(anyhow!("config path:'{}' not exists!", config_path.display()));
            }
            if config_path.is_file() {
                return Err(anyhow!("The configuration path must be a directory!"));
            }
            crate::db::initialize_database(config_path.join("db")).await?;
        }
        crate::sudo()?;
        let home_dir = dirs::home_dir();

        if let Some(home_dir) = home_dir {
            if home_dir.is_dir() {
                let config_dir = home_dir.join(".wgsdc");
                if !config_dir.exists() {
                    tokio::fs::create_dir_all(&config_dir).await?;
                    crate::db::initialize_database(config_dir.join("db")).await?
                }
            }
        }
        Ok(())
    }

    // Non-relay node configuration
    // pub async fn get_peer_config(&mut self, name: &str) -> anyhow::Result<String> {
    //     // node
    //     let node = self.get_by_name(name).await?;

    //     // is relay node
    //     if node.relay {
    //         return Err(anyhow::anyhow!("This function does not support"));
    //     }

    //     // node relay list
    //     let mut node_relay_list = self.list_by_relay(node.relay).await?;

    //     // convert
    //     for v in &mut node_relay_list {
    //         // v.allowed_ips = node.endpoint_allowed_ips.clone();
    //         v.persistent_keepalive = node.persistent_keepalive;
    //     }

    //     let mut lines = String::new();
    //     // node name
    //     lines.push_str(&format!("# {}\n", node.name()));

    //     let interface = Interface::from(node);

    //     // Interface section begins
    //     lines.push_str("[Interface]\n");

    //     // Interface Private key
    //     lines.push_str(&format!("PrivateKey = {}\n", interface.private_key()?));

    //     // Interface address
    //     lines.push_str(&format!("Address = {}\n", interface.address()?));

    //     // Interface listen port, if any
    //     if let Some(listen_port) = interface.listen_port() {
    //         lines.push_str(&format!("ListenPort = {}\n", listen_port));
    //     }

    //     // Interface MTU, if any
    //     if let Some(mtu) = interface.mtu() {
    //         lines.push_str(&format!("MTU = {}\n", mtu));
    //     }

    //     // Interface PreUp, if any
    //     if let Some(pre_up) = interface.pre_up() {
    //         lines.push_str(&format!("PreUp = {}\n", pre_up));
    //     }

    //     // Interface PostUp, if any
    //     if let Some(post_up) = interface.post_up() {
    //         lines.push_str(&format!("PostUp = {}\n", post_up));
    //     }

    //     // Interface PreDown, if any
    //     if let Some(pre_down) = interface.pre_down() {
    //         lines.push_str(&format!("PreDown = {}\n", pre_down));
    //     }

    //     // Interface PostDown, if any
    //     if let Some(post_down) = interface.post_down() {
    //         lines.push_str(&format!("PostDown = {}\n", post_down));
    //     }

    //     // ------------------------------Peer----------------------------------
    //     for node in node_relay_list {
    //         // Peer name
    //         lines.push_str(&format!("# {}\n", node.name()));

    //         let peer = Peer::from(node);

    //         // Peer section begins
    //         lines.push_str("[Peer]\n");

    //         // Peer Public key
    //         lines.push_str(&format!("PublicKey = {}\n", peer.public_key()?));

    //         // Peer Allowed IPs
    //         lines.push_str(&format!("AllowedIPs = {}\n", peer.allowed_ips()?));

    //         // Peer Persistent Keepalive, if any
    //         if let Some(keepalive) = peer.persistent_keepalive() {
    //             lines.push_str(&format!("PersistentKeepalive = {}\n", keepalive));
    //         }

    //         // Peer Endpoint, if any
    //         if let Some(endpoint) = peer.endpoint() {
    //             lines.push_str(&format!("Endpoint = {}\n", endpoint));
    //         }
    //     }

    //     Ok(lines)
    // }

    // Relay node configuration
    // pub async fn get_relay_peer_config(&mut self, node_name: &str) -> anyhow::Result<String> {
    //     // get node
    //     let config_relay_node = self.get_by_name(node_name).await?;

    //     // convert to peer
    //     let node_list = self
    //         .list()
    //         .await?
    //         .iter()
    //         .filter(|n| n.name().ne(node_name))
    //         .map(|v| v.clone())
    //         .collect::<Vec<Node>>();

    //     let mut lines = String::new();
    //     // node name
    //     lines.push_str(&format!("# {}\n", config_relay_node.name()));

    //     let interface = Interface::from(config_relay_node);

    //     // Interface section begins
    //     lines.push_str("[Interface]\n");

    //     // Interface Private key
    //     lines.push_str(&format!("PrivateKey = {}\n", interface.private_key()?));

    //     // Interface address
    //     lines.push_str(&format!("Address = {}\n", interface.address()?));

    //     // Interface listen port, if any
    //     if let Some(listen_port) = interface.listen_port() {
    //         lines.push_str(&format!("ListenPort = {}\n", listen_port));
    //     }

    //     // MTU, if any
    //     if let Some(mtu) = interface.mtu() {
    //         lines.push_str(&format!("MTU = {}\n", mtu));
    //     }

    //     // PreUp, if any
    //     if let Some(pre_up) = interface.pre_up() {
    //         lines.push_str(&format!("PreUp = {}\n", pre_up));
    //     }

    //     // PostUp, if any
    //     if let Some(post_up) = interface.post_up() {
    //         lines.push_str(&format!("PostUp = {}\n", post_up));
    //     }

    //     // PreDown, if any
    //     if let Some(pre_down) = interface.pre_down() {
    //         lines.push_str(&format!("PreDown = {}\n", pre_down));
    //     }

    //     // PostDown, if any
    //     if let Some(post_down) = interface.post_down() {
    //         lines.push_str(&format!("PostDown = {}\n", post_down));
    //     }

    //     // ------------------------------Peer----------------------------------
    //     for node in node_list {
    //         // node name
    //         lines.push_str(&format!("# {}\n", node.name()));

    //         let peer = Peer::from(node);

    //         // Peer section begins
    //         lines.push_str("[Peer]\n");

    //         // Peer Public key
    //         lines.push_str(&format!("PublicKey = {}\n", peer.public_key()?));

    //         // Peer Allowed IPs
    //         lines.push_str(&format!("AllowedIPs = {}\n", peer.allowed_ips()?));

    //         // Peer Persistent Keepalive
    //         if let Some(keepalive) = peer.persistent_keepalive() {
    //             lines.push_str(&format!("PersistentKeepalive = {}\n", keepalive));
    //         }
    //     }
    //     return Ok(lines);
    // }

}