use std::net::SocketAddr;

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    
    loop {
        println!("meh1");
        // The second item contains the IP and port of the new connection.
        match listener.accept().await{
            Ok((stream, addr)) =>  handle(stream, addr).await,
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}


async fn handle(mut stream : TcpStream, addr:  SocketAddr) {
    stream.write_all(&addr.stocazzoo()).await.unwrap();
    println!("meh2 {}", stream.read_to_string(&mut String::new()).await.unwrap());
}

trait ASD{
    fn stocazzoo(&self) -> Vec<u8>;
}


impl ASD for SocketAddr {
    fn stocazzoo(&self) -> Vec<u8> {
        println!("ppp");
        format!("{:?}\n", self).into_bytes()
    }
}