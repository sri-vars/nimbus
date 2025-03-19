use std::{
    fs,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NimbusClusterConfiguration {
    pub cluster: ClusterConfiguration,
    pub nodes: Vec<NodeConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClusterConfiguration {
    pub cluster_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NodeConfiguration {
    pub id: u64,
    pub private_transport: TransportConfiguration,
    pub public_transport: TransportConfiguration,
    pub working_directory: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TransportConfiguration {
    pub host: IpAddr,
    pub port: u16,
}

impl NimbusClusterConfiguration {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file_content = fs::read_to_string(path)?;
        Ok(serde_yml::from_str(file_content.as_str())?)
    }
}

impl TransportConfiguration {
    pub fn sock_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_cluster_configuration() {
        let test_cluster_configuration = "../example/configuration/cluster.yml";
        let nimbus_cluster_configuration =
            NimbusClusterConfiguration::new(test_cluster_configuration)
                .expect("unable to read configuration file");
        println!("{:#?}", nimbus_cluster_configuration);
    }
}
