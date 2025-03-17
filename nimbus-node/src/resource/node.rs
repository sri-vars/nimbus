use std::sync::Arc;

use nimbus_conf::{NimbusClusterConfiguration, NodeConfiguration};
use nimbus_rt::lock::RwLock;

use crate::Result;

pub struct Node {
    pub global_config: Arc<RwLock<NimbusClusterConfiguration>>,
    pub node_config: Arc<RwLock<NodeConfiguration>>,
}

impl Node {
    pub fn new(config: NimbusClusterConfiguration, node: u64) -> Result<Self> {
        let node_config = Arc::new(RwLock::new(
            config
                .nodes
                .iter()
                .find(|node_config| node_config.id == node)
                .expect("node is not present in configuration")
                .clone(),
        ));
        let global_config = Arc::new(RwLock::new(config));
        Ok(Self {
            global_config,
            node_config,
        })
    }
}
