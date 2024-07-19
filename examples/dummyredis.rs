use std::net::SocketAddr;

use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis Listening on: {}", addr);
    loop {
        let (socket, raddr) = listener.accept().await?;
        info!("Accept connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_con(socket, raddr).await {
                warn!("Error: {:?}", e);
            }
        });
    }
}

async fn process_redis_con(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                info!("Read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("Read: {:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                info!("Error: {:?}", e);
                break;
            }
        }
    }
    warn!("Connection closed,raddr:{}", raddr);
    Ok(())
}
