use std::{path::PathBuf, sync::Arc};

use async_trait::async_trait;
use nimbus_conf::NimbusClusterConfiguration;
use nimbus_rt::{Task, futures::join, rt::NimbusRt};

use crate::{
    Node, Result,
    resource::transport::{TcpTransport, TransportStreamTrait, TransportTrait},
};

pub fn node_start(config: &PathBuf, node: u64) -> Result<()> {
    let nc_config = NimbusClusterConfiguration::new(config)?;
    let node = Arc::new(Node::new(nc_config, node)?);

    let rt = NimbusRt::instance();
    rt.run(async move {
        let node_config = node.node_config.read().await;
        let public_transport = TcpTransport::new(node_config.public_transport.sock_addr())
            .await
            .unwrap();
        let private_transport = TcpTransport::new(node_config.private_transport.sock_addr())
            .await
            .unwrap();

        let public_task = PublicNodeTask::new(node.clone(), public_transport).await;
        let private_task = PrivateNodeTask::new(node.clone(), private_transport).await;

        // Get the task objects
        let public_task_handle = public_task.run();
        let private_task_handle = private_task.run();

        // Await the inner results separately
        let (public_task_res, private_task_res) =
            join!(public_task_handle.await, private_task_handle.await);

        println!("{:#?}", public_task_res);
        println!("{:#?}", private_task_res);
    });
    Ok(())
}

pub struct PublicNodeTask<T: TransportTrait> {
    pub node: Arc<Node>,
    pub transport: Arc<T>,
}

pub struct PrivateNodeTask<T: TransportTrait> {
    pub node: Arc<Node>,
    pub transport: Arc<T>,
}

impl<T: TransportTrait> PublicNodeTask<T> {
    pub async fn new(node: Arc<Node>, transport: T) -> Self {
        Self {
            node,
            transport: Arc::new(transport),
        }
    }
}

impl<T: TransportTrait> PrivateNodeTask<T> {
    pub async fn new(node: Arc<Node>, transport: T) -> Self {
        Self {
            node,
            transport: Arc::new(transport),
        }
    }
}

#[async_trait]
pub trait NodeTaskTrait {
    type TaskType;
    async fn run(&self) -> Task<Self::TaskType>;
}

#[async_trait]
impl<T: TransportTrait + Send + Sync + 'static> NodeTaskTrait for PublicNodeTask<T> {
    type TaskType = Result<()>;

    async fn run(&self) -> Task<Self::TaskType> {
        let transport = self.transport.clone();
        NimbusRt::spawn(async move { echo_server(transport).await })
    }
}

#[async_trait]
impl<T: TransportTrait + Send + Sync + 'static> NodeTaskTrait for PrivateNodeTask<T> {
    type TaskType = Result<()>;

    async fn run(&self) -> Task<Self::TaskType> {
        let transport = self.transport.clone();
        NimbusRt::spawn(async move { echo_server(transport).await })
    }
}

async fn echo_server<T: TransportTrait + Send + Sync + 'static>(transport: Arc<T>) -> Result<()> {
    let transport_clone = transport.clone();
    let mut tasks = vec![];
    loop {
        match transport_clone.accept().await {
            Ok(transport_stream) => {
                println!("New connection from {}", transport_stream.peer_addr());
                let transport_stream = Arc::new(transport_stream);
                let task = NimbusRt::spawn(handle_client(transport_stream));
                tasks.push(task);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {:?}", e);
            }
        }
        tasks.retain(|task| !task.is_finished());
    }
}

async fn handle_client<T: TransportStreamTrait + Send + Sync + 'static>(
    transport_stream: Arc<T>,
) -> Result<()> {
    let mut buffer = vec![0u8; 1024];

    loop {
        let bytes_read = match transport_stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", transport_stream.peer_addr());
                return Ok(());
            }
            Ok(n) => n,
            Err(e) => return Err(e.into()),
        };

        if let Err(e) = transport_stream.write(&buffer[..bytes_read]).await {
            println!("Client {} disconnected", transport_stream.peer_addr());
            return Err(e.into());
        }
    }
}
