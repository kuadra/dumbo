mod buffo;
use std::io::Result;

use buffo::Buffo;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let bound = TcpListener::bind("127.0.0.1:6379");
    let listener = bound.await.unwrap();
    let mut buffer = Buffo::new();

    loop{
        println!("===LOOP===");
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle(stream, buffer).await;
        });
    }
}

async fn handle(mut stream: TcpStream, mut buffer: Buffo) -> Buffo{
    stream.readable().await.unwrap();
    match stream.try_read(buffer.get_mem()) {
        Ok(n) => {
            println!("Read {} bytes", n);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    let response = format!("OK!");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    println!("STREAM FLUSHED");
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    println!("Buffer A: {:?}", String::from_utf8(buffer.get_mem().to_vec()));
    buffer
}
