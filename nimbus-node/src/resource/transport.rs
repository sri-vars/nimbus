use async_trait::async_trait;
use nimbus_rt::{
    futures::{AsyncReadExt, AsyncWriteExt},
    lock::Mutex,
    net::{TcpListener, TcpStream},
};
use std::{net::SocketAddr, sync::Arc};

use crate::Result;

pub enum Transport {
    Tcp(TcpTransport),
}

#[derive(Debug)]
pub struct TcpTransport {
    pub listener: TcpListener,
}

pub struct TcpTransportStream {
    pub client_addr: SocketAddr,
    pub inner: Mutex<TcpStreamInner>,
}

pub struct TcpStreamInner {
    pub stream: TcpStream,
}

#[async_trait]
pub trait TransportTrait: Send + Sync {
    type Stream: TransportStreamTrait + Send + Sync;
    async fn accept(&self) -> Result<Self::Stream>;
}

#[async_trait]
pub trait TransportStreamTrait: Send + Sync {
    fn peer_addr(&self) -> SocketAddr;
    async fn read(&self, buffer: &mut [u8]) -> Result<usize>;
    async fn write(&self, data: &[u8]) -> Result<usize>;
}

impl TcpTransport {
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        let listener = TcpListener::bind(&addr).await?;
        Ok(Self { listener })
    }
}

#[async_trait]
impl TransportTrait for TcpTransport {
    type Stream = Arc<TcpTransportStream>;

    async fn accept(&self) -> Result<Self::Stream> {
        let (stream, client_addr) = self.listener.accept().await?;
        Ok(Arc::new(TcpTransportStream::new(stream, client_addr)))
    }
}

impl TcpTransportStream {
    pub fn new(stream: TcpStream, client_addr: SocketAddr) -> Self {
        Self {
            client_addr,
            inner: Mutex::new(TcpStreamInner { stream }),
        }
    }
}

#[async_trait]
impl TransportStreamTrait for Arc<TcpTransportStream> {
    fn peer_addr(&self) -> SocketAddr {
        self.client_addr
    }

    async fn read(&self, buffer: &mut [u8]) -> Result<usize> {
        let mut inner = self.inner.lock().await;
        let bytes_read = inner.stream.read(buffer).await?;
        Ok(bytes_read)
    }

    async fn write(&self, data: &[u8]) -> Result<usize> {
        let mut inner = self.inner.lock().await;
        let bytes_written = inner.stream.write(data).await?;
        Ok(bytes_written)
    }
}
