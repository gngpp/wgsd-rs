extern crate ipnet;

use crate::conf::endpoint::Node;
use crate::conf::model::WireGuard;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod endpoint;
mod model;

#[async_trait]
pub trait RW {
    // get node server
    async fn get(&mut self) -> anyhow::Result<Node>;

    // set node server
    async fn set(&mut self, node: Node) -> anyhow::Result<()>;

    // push node to list
    async fn push(&mut self, node: Node) -> anyhow::Result<()>;

    // remove node from list
    async fn remove_for_name(&mut self, node_name: &str) -> anyhow::Result<()>;

    // remove node from list
    async fn remove(&mut self, index: usize) -> anyhow::Result<()>;

    // get from node list
    async fn list(&mut self) -> anyhow::Result<Vec<Node>>;

    // exist node(exclude node server)
    async fn exist(&self, name: String) -> bool;
}

const DEFAULT_PATH: &str = "/etc/wireguard/wgsdc";

pub struct Configuration {
    path: PathBuf,
    wireguard: Arc<Mutex<WireGuard>>,
}

impl Configuration {
    async fn init(conf: String) -> anyhow::Result<PathBuf> {
        // example: wg0
        if conf.is_empty() {
            return Err(anyhow!("{} cannot been empty!", conf));
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
        let path_buf = path_buf.join(format!("{}.yaml", conf));
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

    async fn read(path: &PathBuf) -> anyhow::Result<WireGuard> {
        log::debug!("ready to read configuration file: {}", path.display());
        let string = tokio::fs::read_to_string(path).await.context(format!(
            "Error reading {} configuration file",
            path.display()
        ))?;
        serde_yaml::from_str(string.as_str()).context("Serialized read configuration failed")
    }

    async fn write(path: &PathBuf, wg: &WireGuard) -> anyhow::Result<()> {
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
impl RW for Configuration {
    async fn get(&mut self) -> anyhow::Result<Node> {
        self.wireguard.lock().await.get().await
    }

    async fn set(&mut self, node: Node) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.set(node).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn push(&mut self, node: Node) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.push(node).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn remove_for_name(&mut self, node_name: &str) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.remove_for_name(node_name).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn remove(&mut self, index: usize) -> anyhow::Result<()> {
        let mut wg = self.wireguard.lock().await;
        wg.remove(index).await?;
        Configuration::write(&self.path, &wg).await
    }

    async fn list(&mut self) -> anyhow::Result<Vec<Node>> {
        self.wireguard.lock().await.list().await
    }

    async fn exist(&self, name: String) -> bool {
        self.wireguard.lock().await.exist(name).await
    }
}
