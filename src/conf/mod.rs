extern crate ipnet;

use crate::conf::endpoint::{Interface, Node, Peer};
use crate::conf::model::WireGuard;
use anyhow::{anyhow, Context};
use async_trait::async_trait;

use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod endpoint;
mod model;

#[async_trait]
pub trait NodeOpt: Sized {
    // get node relay
    async fn get_relay(&mut self) -> anyhow::Result<Node>;

    // get node by name(exclude node server)
    async fn get_by_name(&mut self, node_name: &str) -> anyhow::Result<Node>;

    // set node server
    async fn set_relay(&mut self, node: Node) -> anyhow::Result<()>;

    // push node to list
    async fn push(&mut self, node: Node) -> anyhow::Result<()>;

    // get from node list
    async fn list(&mut self) -> anyhow::Result<Vec<Node>>;

    // clear node(include node server)
    async fn clear(&mut self) -> anyhow::Result<()>;

    // remove all from list
    async fn remove_all(&mut self) -> anyhow::Result<()>;

    // remove node from list
    async fn remove_by_name(&mut self, node_name: &str) -> anyhow::Result<()>;

    // remove node from list
    async fn remove_by_index(&mut self, index: usize) -> anyhow::Result<()>;

    // drop
    async fn drop(&mut self) -> anyhow::Result<()>;

    // exist node(exclude node server)
    async fn exist(&self, name: String) -> bool;
}

#[async_trait]
pub trait AsyncTryFrom<T>: Sized {
    type Error;

    async fn try_from(_: T) -> Result<Self, Self::Error>;
}

const DEFAULT_PATH: &str = "/etc/wireguard/wgsdc";
const DEFAULT_FILE_SUFFIX: &str = ".yaml";

pub struct Configuration {
    path: PathBuf,
    wireguard: Arc<Mutex<WireGuard>>,
}

impl Configuration {
    async fn init(conf: String) -> anyhow::Result<PathBuf> {
        // example: wg0
        if conf.is_empty() {
            return Err(anyhow!("config name:'{}' cannot been empty!", conf));
        }
        crate::sudo()?;
        let path_buf = PathBuf::from(DEFAULT_PATH);

        // create dir: /etc/wireguard/wgsdc
        if !path_buf.exists() {
            tokio::fs::create_dir_all(&path_buf).await?;
            log::debug!("The {} directory has been created", DEFAULT_PATH);
        } else {
            if !path_buf.is_dir() {
                return Err(anyhow::anyhow!("{} not is dir!", DEFAULT_PATH));
            }
            log::debug!("The {} directory exists", DEFAULT_PATH);
        }
        // create file: /etc/wireguard/wgsdc/wg0.yaml
        let path_buf = path_buf.join(format!("{}{}", conf, DEFAULT_FILE_SUFFIX));
        if !path_buf.exists() {
            log::debug!("ready to create: {}", path_buf.display());
            tokio::fs::File::create(&path_buf).await?;
        }

        Ok(path_buf)
    }

    pub async fn print_std(&self) -> anyhow::Result<()> {
        let wgsdc = Self::read(&self.path).await?;
        println!("{}", serde_yaml::to_string(&wgsdc)?);
        Ok(())
    }

    // node configuration string
    pub async fn get_peer_config(&mut self, name: &str) -> anyhow::Result<String> {
        // node
        let node = self.get_by_name(name).await?;
        // node relay
        let mut node_relay = self.get_relay().await?;

        // convert
        node_relay.allowed_ips = node.endpoint_allowed_ips.clone();
        node_relay.persistent_keepalive = node.persistent_keepalive;

        let mut lines = String::new();
        // node name
        lines.push_str(&format!("# {}\n", node.name()));

        let interface = Interface::from(node);

        // Interface section begins
        lines.push_str("[Interface]\n");

        // Interface Private key
        lines.push_str(&format!("PrivateKey = {}\n", interface.private_key()?));

        // Interface address
        lines.push_str(&format!("Address = {}\n", interface.address()?));

        // Interface listen port, if any
        if let Some(listen_port) = interface.listen_port() {
            lines.push_str(&format!("ListenPort = {}\n", listen_port));
        }

        // Interface MTU, if any
        if let Some(mtu) = interface.mtu() {
            lines.push_str(&format!("MTU = {}\n", mtu));
        }

        // Interface PreUp, if any
        if let Some(pre_up) = interface.pre_up() {
            lines.push_str(&format!("PreUp = {}\n", pre_up));
        }

        // Interface PostUp, if any
        if let Some(post_up) = interface.post_up() {
            lines.push_str(&format!("PostUp = {}\n", post_up));
        }

        // Interface PreDown, if any
        if let Some(pre_down) = interface.pre_down() {
            lines.push_str(&format!("PreDown = {}\n", pre_down));
        }

        // Interface PostDown, if any
        if let Some(post_down) = interface.post_down() {
            lines.push_str(&format!("PostDown = {}\n", post_down));
        }

        // ------------------------------Peer----------------------------------
        // Peer name
        lines.push_str(&format!("# {}\n", node_relay.name()));

        let peer = Peer::from(node_relay);

        // Peer section begins
        lines.push_str("[Peer]\n");

        // Peer Public key
        lines.push_str(&format!("PublicKey = {}\n", peer.public_key()?));

        // Peer Allowed IPs
        lines.push_str(&format!("AllowedIPs = {}\n", peer.allowed_ips()?));

        // Peer Persistent Keepalive, if any
        if let Some(keepalive) = peer.persistent_keepalive() {
            lines.push_str(&format!("PersistentKeepalive = {}\n", keepalive));
        }

        // Peer Endpoint, if any
        if let Some(endpoint) = peer.endpoint() {
            lines.push_str(&format!("Endpoint = {}\n", endpoint));
        }

        Ok(lines)
    }

    // node relay configuration string
    pub async fn get_peer_relay_config(&mut self) -> anyhow::Result<String> {
        let node_relay = self.get_relay().await?;
        let mut lines = String::new();
        // node name
        lines.push_str(&format!("# {}\n", node_relay.name()));

        let interface = Interface::from(node_relay);

        // Interface section begins
        lines.push_str("[Interface]\n");

        // Interface Private key
        lines.push_str(&format!("PrivateKey = {}\n", interface.private_key()?));

        // Interface address
        lines.push_str(&format!("Address = {}\n", interface.address()?));

        // Interface listen port, if any
        if let Some(listen_port) = interface.listen_port() {
            lines.push_str(&format!("ListenPort = {}\n", listen_port));
        }

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

        // ------------------------------Peer----------------------------------
        for node in self.list().await? {
            // node name
            lines.push_str(&format!("# {}\n", node.name()));

            let peer = Peer::from(node);

            // Peer section begins
            lines.push_str("[Peer]\n");

            // Peer Public key
            lines.push_str(&format!("PublicKey = {}\n", peer.public_key()?));

            // Peer Allowed IPs
            lines.push_str(&format!("AllowedIPs = {}\n", peer.allowed_ips()?));

            // Peer Persistent Keepalive
            if let Some(keepalive) = peer.persistent_keepalive() {
                lines.push_str(&format!("PersistentKeepalive = {}\n", keepalive));
            }
        }
        return Ok(lines);
    }

    async fn read(path: &PathBuf) -> anyhow::Result<WireGuard> {
        crate::sudo()?;
        log::debug!("ready to read configuration file: {}", path.display());
        let string = tokio::fs::read_to_string(path).await.context(format!(
            "Error reading {} configuration file",
            path.display()
        ))?;
        serde_yaml::from_str(string.as_str()).context("Serialized read configuration failed")
    }

    async fn write(path: &PathBuf, wg: &WireGuard) -> anyhow::Result<()> {
        crate::sudo()?;
        log::debug!("ready to write configuration files to: {}", path.display());
        let str = serde_yaml::to_string(wg).context("Serialized write configuration failed")?;
        tokio::fs::write(path, str)
            .await
            .context(format!("Error writing to {} config file", path.display()))
    }

    pub async fn new(conf: String) -> anyhow::Result<Self> {
        let path = Self::init(conf)
            .await
            .context("Initial configuration failed")?;
        let wire_guard = Self::read(&path).await?;
        let configuration = Self {
            path,
            wireguard: Arc::new(Mutex::new(wire_guard)),
        };
        Ok(configuration)
    }
}

#[async_trait]
impl AsyncTryFrom<String> for Configuration {
    type Error = anyhow::Error;

    async fn try_from(mut config: String) -> Result<Self, Self::Error> {
        config.push_str(DEFAULT_FILE_SUFFIX);
        let buf_path = PathBuf::from(DEFAULT_PATH).join(&config);
        if !buf_path.exists() {
            return Err(anyhow::anyhow!("configuration: {} does not exist!", config));
        }
        let wire_guard = Self::read(&buf_path).await?;
        let configuration = Self {
            path: buf_path,
            wireguard: Arc::new(Mutex::new(wire_guard)),
        };
        Ok(configuration)
    }
}

#[async_trait]
impl NodeOpt for Configuration {
    async fn get_relay(&mut self) -> anyhow::Result<Node> {
        self.wireguard.lock().await.get_relay().await
    }

    async fn get_by_name(&mut self, node_name: &str) -> anyhow::Result<Node> {
        let mut wg = self.wireguard.lock().await;
        wg.get_by_name(node_name).await
    }

    async fn set_relay(&mut self, node: Node) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.set_relay(node).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn push(&mut self, node: Node) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.push(node).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn list(&mut self) -> anyhow::Result<Vec<Node>> {
        self.wireguard.lock().await.list().await
    }

    async fn clear(&mut self) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.clear().await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn remove_all(&mut self) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.remove_all().await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn remove_by_name(&mut self, node_name: &str) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.remove_by_name(node_name).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn remove_by_index(&mut self, index: usize) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.remove_by_index(index).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn drop(&mut self) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        NodeOpt::drop(wg.deref_mut()).await?;
        tokio::fs::remove_file(&self.path).await.context(format!(
            "Delete configuration file: {}, an error occurred",
            self.path.display()
        ))
    }

    async fn exist(&self, name: String) -> bool {
        self.wireguard.lock().await.exist(name).await
    }
}
