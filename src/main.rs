use std::net::SocketAddr;

use tokio::net::{TcpListener};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("0.0.0.0:6379").await.unwrap();
    
    loop {
        // The second item contains the IP and port of the new connection.
        match listener.accept().await{
            Ok((mut socket, _addr)) =>  socket.write_all(&write_address2(_addr)).await.unwrap(),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

fn write_address2(addr : SocketAddr) -> Vec<u8> {
    format!("{}\n", addr).into_bytes()
}