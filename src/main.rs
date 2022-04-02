use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    match TcpListener::bind("127.0.0.1:6379").await {
        Ok(listener) => handle_connection(listener).await,
        Err(e) => println!("{}", e),
    }
}

async fn handle_connection(listener: TcpListener) {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => handle_stream(stream).await,
            Err(e) => println!("{}", e),
        }
    }
}

async fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    stream.read(&mut buffer).await.unwrap();
    let data = match std::str::from_utf8(&buffer[0..4096]) {
        Ok(data) => data,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("{}", data);
}
