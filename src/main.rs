mod buffo;
use std::io::Result;

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};


#[tokio::main]
async fn main() {
    let bound = TcpListener::bind("127.0.0.1:6379");
    let listener = bound.await.unwrap();
    let buffer = Buffo::new();

    //buffer_scrittura
    wait_for_data(listener, buffer).await.unwrap();
    
    //scrivi data(buffer scrittura)
}

async fn wait_for_data(listener: TcpListener, buffer: Buffo) -> Result<String> {
    loop {
        println!("===LOOP===");
        let connection = listener.accept().await;
        match connection {
            Ok((stream, addr)) => {
                println!("New client: {:?}", addr);
                tokio::spawn(async move {
                    handle(stream).await;
                });
            }
            Err(err) => return Err(err),
        }
    }
}

async fn handle(mut stream: TcpStream) {
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

    let response = format!("OK!");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    println!("STREAM FLUSHED");
    tokio::time::sleep(std::time::Duration::from_millis(5000)).await;
    println!("Buffer A: {:?}", String::from_utf8(buf.get_mem().to_vec()));
}
