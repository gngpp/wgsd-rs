extern crate ipnet;

use crate::conf::model::WG;
use anyhow::{anyhow, Context};
use std::path::PathBuf;

pub mod model;

const DEFAULT_PATH: &str = "/etc/wireguard/wgsdc";

pub struct Configuration {
    path: PathBuf,
}

impl Configuration {

    pub fn init(conf: String) -> anyhow::Result<PathBuf> {
        // example: wg0
        if conf.is_empty() {
            return Err(anyhow!("{} cannot been empty!", conf));
        }
        crate::sudo()?;
        let path_buf = PathBuf::from_iter(vec![DEFAULT_PATH, &conf]);

        // create dir: /etc/wireguard/wgsdc/wg0
        if !path_buf.exists() {
            std::fs::create_dir_all(&path_buf)?;
            log::debug!("The {} directory has been created", DEFAULT_PATH);
        } else {
            if !path_buf.is_dir() {
                return Err(anyhow::anyhow!("{} not is dir!", DEFAULT_PATH));
            }
            log::debug!("The {} directory exists", DEFAULT_PATH);
        }
        // create file: /etc/wireguard/wgsdc/wg0/wg0.yaml
        let yaml_buf = path_buf.join(format!("{}.yaml", conf));
        if !yaml_buf.exists() {
            let file = std::fs::File::create(yaml_buf)?;
            println!("{:?}", file);
        }
        Ok(path_buf)
    }

    pub async fn print_std(_p: PathBuf) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn read(&self) -> anyhow::Result<WG> {
        let string = tokio::fs::read_to_string(&self.path)
            .await
            .context("Read configuration error!")?;
        serde_yaml::from_str(string.as_str()).context("Serialized read configuration failed")
    }

    pub async fn write(&self, wg: WG) -> anyhow::Result<()> {
        let str = serde_yaml::to_string(&wg).context("Serialized write configuration failed")?;
        tokio::fs::write(&self.path, str).await?;
        Ok(())
    }

    pub fn new(conf: String) -> Configuration {

        Self { path: Configuration::init(conf).expect("Initial configuration failed") }
    }

}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::new(String::from("wg0"))
    }
}

pub trait NodeProperty {
    fn get_description(&self) -> String;

    fn get_mtu(&self) -> u16;
}