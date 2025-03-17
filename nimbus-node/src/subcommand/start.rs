use std::path::PathBuf;

use nimbus_conf::NimbusClusterConfiguration;

use crate::{Node, Result};

pub fn node_start(config: &PathBuf, node: u64) -> Result<()> {
    let nc_config = NimbusClusterConfiguration::new(config)?;
    let node = Node::new(nc_config, node)?;
    Ok(())
}
