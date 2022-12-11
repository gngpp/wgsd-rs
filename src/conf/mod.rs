extern crate ipnet;

use crate::conf::model::WGSDC;
use anyhow::{anyhow, Context};
use std::path::PathBuf;

pub mod endpoint;
pub mod model;

const DEFAULT_PATH: &str = "/etc/wireguard/wgsdc";

pub struct Configuration {
    path: PathBuf,
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
        let wgsdc = self.read().await?;
        println!("{}", serde_yaml::to_string(&wgsdc)?);
        Ok(())
    }

    pub async fn read(&self) -> anyhow::Result<WGSDC> {
        log::debug!("ready to read configuration file: {}", self.path.display());
        let string = tokio::fs::read_to_string(&self.path)
            .await
            .context("Read configuration error!")?;
        serde_yaml::from_str(string.as_str()).context("Serialized read configuration failed")
    }

    pub async fn write(&self, wg: WGSDC) -> anyhow::Result<()> {
        log::debug!(
            "ready to write configuration files to: {}",
            self.path.display()
        );
        let str = serde_yaml::to_string(&wg).context("Serialized write configuration failed")?;
        tokio::fs::write(&self.path, str).await?;
        Ok(())
    }

    pub async fn new(conf: String) -> anyhow::Result<Self> {
        let configuration = Self {
            path: Configuration::init(conf)
                .await
                .context("Initial configuration failed")?,
        };
        Ok(configuration)
    }
}
