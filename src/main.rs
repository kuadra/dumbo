mod buffo;
use std::io::Result;

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};

use crate::buffo::Buffo;


#[tokio::main]
async fn main() {
    let bound = TcpListener::bind("127.0.0.1:6379");
    let listener = bound.await.unwrap();
    let mut buffer = Buffo::new();

    loop{
        println!("===LOOP===");
        let connection = listener.accept().await;
        match connection {
            Ok((stream, addr)) => {
                println!("New client: {:?}", addr);
                tokio::spawn(async move {
                    handle(stream).await;
                });
            }
            Err(err) => println!("{}", err),
        }
    }

async fn handle(mut stream: TcpStream) {
    stream.readable().await.unwrap();
    let mut buf : buffo::Buffo = buffo::Buffo::new();
    match stream.try_read(buf.get_mem()) {
        Ok(n) => println!("Read {} bytes", n),
        Err(e) => println!("Error: {:?}", e),
    }

    let response = format!("OK!");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    println!("STREAM FLUSHED");
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    println!("Buffer A: {:?}", String::from_utf8(buf.get_mem().to_vec()));
}
