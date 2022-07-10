mod buffo;
use std::io::Result;

use buffo::Buffo;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let bound = TcpListener::bind("127.0.0.1:6379");
    let listener = bound.await.unwrap();
    let buffer = Buffo::new();

    wait_for_data(listener, buffer).await.unwrap();
    
    //crunch_data()
}

async fn wait_for_data(listener: TcpListener, buffer: Buffo) -> Result<String> {
    loop {
        println!("===LOOP===");
        let connection = listener.accept().await;
        match connection {
            Ok((stream, addr)) => {
                println!("New client: {:?}", addr);
                handle(stream, buffer).await;
                return Ok("Appost".to_string());
            }
            Err(err) => return Err(err),
        }
    }
}

async fn handle(stream: TcpStream, mut buffer : Buffo) -> u8 {
    stream.readable().await.unwrap();
    match stream.try_read(buffer.get_mem()) {
        Ok(n) => {
            println!("Read {} bytes", n);
            println!("Buffer A: {:?}", String::from_utf8(buffer.get_mem().to_vec()));
            return 1;
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return 0;
        }
    }
}
