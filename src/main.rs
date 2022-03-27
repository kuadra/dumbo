use std::error::Error;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    match TcpListener::bind("127.0.0.1:6379").await {
        Ok(res) => handle_connection(res).await,
        Err(err) => println!("Binding error"),
    }
}

async fn handle_connection(listener: TcpListener) {
    println!("{:?}", listener);
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => handle_stream(stream).await.unwrap(),
            Err(err) => println!("Binding error"),
        }
    }
}

async fn handle_stream(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        // Creating the buffer **after** the `await` prevents it from
        // being stored in the async task.
        let mut buf = [0; 4096];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}
