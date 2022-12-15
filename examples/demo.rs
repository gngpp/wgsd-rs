use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

struct Configuration {
    path: PathBuf,
    wireguard: Arc<Mutex<WireGuard>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct WireGuard {
    value: Option<String>
}

impl Configuration {

    //noinspection DuplicatedCode
    async fn init(conf: String) -> anyhow::Result<PathBuf> {
        // example: wg0
        if conf.is_empty() {
            return Err(anyhow!("config name:'{}' cannot been empty!", conf));
        }
        sudo::escalate_if_needed().unwrap();
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

    //noinspection DuplicatedCode
    async fn read(path: &PathBuf) -> anyhow::Result<WireGuard> {
        sudo::escalate_if_needed().unwrap();
        log::debug!("ready to read configuration file: {}", path.display());
        let string = tokio::fs::read_to_string(path).await.context(format!(
            "Error reading {} configuration file",
            path.display()
        ))?;
        serde_yaml::from_str(string.as_str()).context("Serialized read configuration failed")
    }

    pub async fn new(conf: String) -> anyhow::Result<Self> {
        let path = Self::init(conf)
            .await
            .context("Initial configuration failed")?;
        let wire_guard = Configuration::read(&path).await?;
        let configuration = Self {
            path,
            wireguard: Arc::new(Mutex::new(wire_guard)),
        };
        Ok(configuration)
    }

}

const DEFAULT_PATH: &str = "/etc/wireguard/wgsdc";
const DEFAULT_FILE_SUFFIX: &str = ".yaml";

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    const PEER: &str = "peer";
    const PEER_SERVER: &str = "peer-server";
    let node_type_option = vec![PEER, PEER_SERVER];
    let node_type_select = inquire::Select::new("Select the peer node type that needs to be revoked.", node_type_option).prompt();
    println!("start");
    match node_type_select {
        Ok(node_type) => {
            match node_type {
                PEER => {
                    let _configuration = Configuration::new("test".to_string()).await?;
                    println!("await")
                }
                PEER_SERVER => {

                }
                _ => {}
            }
        }
        Err(_) => {}
    }
    Ok(())
}