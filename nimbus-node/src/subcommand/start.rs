use std::{collections::VecDeque, net::SocketAddr, path::PathBuf, sync::Arc};

use nimbus_conf::NimbusClusterConfiguration;
use nimbus_rt::{
    Task,
    futures::{AsyncReadExt, join},
    io::AsyncWriteExt,
    lock::Mutex,
    net::TcpListener,
    rt::NimbusRt,
};

use crate::{Error, Node, Result};

pub fn node_start(config: &PathBuf, node: u64) -> Result<()> {
    let nc_config = NimbusClusterConfiguration::new(config)?;
    let node = Arc::new(Node::new(nc_config, node)?);

    let rt = NimbusRt::instance();
    rt.run(async {
        let public_task = NimbusRt::spawn(public_task(node.clone()));
        let private_task = NimbusRt::spawn(private_task(node));

        let (public_task_res, private_task_res) = join!(public_task, private_task);

        if let Err(e) = public_task_res {
            eprintln!("Public task failed: {:?}", e);
        }
        if let Err(e) = private_task_res {
            eprintln!("Private task failed: {:?}", e);
        }
    });
    Ok(())
}

async fn public_task(node: Arc<Node>) -> Result<()> {
    let node_config = node.node_config.read().await;
    let sock_addr = node_config.public_transport.sock_addr();
    echo_server(sock_addr).await
}

async fn private_task(node: Arc<Node>) -> Result<()> {
    let node_config = node.node_config.read().await;
    let sock_addr = node_config.private_transport.sock_addr();
    echo_server(sock_addr).await
}

async fn echo_server(addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on {}", addr);
    let tasks = Arc::new(Mutex::new(VecDeque::<Task<Result<()>>>::new()));
    let tasks_clone = tasks.clone();
    let accept_task = NimbusRt::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, peer_addr)) => {
                    println!("New connection from {}", peer_addr);
                    let handle = NimbusRt::spawn(handle_client(socket, peer_addr));
                    let mut tasks_guard = tasks_clone.lock().await;
                    tasks_guard.push_back(handle);
                    while let Some(task) = tasks_guard.front() {
                        if task.is_finished() {
                            let task = tasks_guard.pop_front().unwrap();
                            drop(tasks_guard);
                            if let Err(e) = task.await {
                                eprintln!("Task error: {:?}", e);
                            }
                            tasks_guard = tasks_clone.lock().await;
                        } else {
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {:?}", e);
                }
            }
        }
    });
    accept_task.await;
    Ok(())
}
async fn handle_client(mut socket: nimbus_rt::net::TcpStream, peer_addr: SocketAddr) -> Result<()> {
    let mut buffer = vec![0u8; 1024];

    loop {
        let bytes_read = match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", peer_addr);
                return Ok(());
            }
            Ok(n) => n,
            Err(e) => return Err(Error::RuntimeIoError(e)),
        };

        if let Err(e) = socket.write_all(&buffer[..bytes_read]).await {
            eprintln!("Error writing to client {}: {:?}", peer_addr, e);
        }
    }
}
