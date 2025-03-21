use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("nimbus_conf error: {}", _0.to_string())]
    NimbusConf(#[from] nimbus_conf::Error),
    #[error("nimbus_node error: {}", _0.to_string())]
    NimbusNode(#[from] nimbus_node::Error),
}
