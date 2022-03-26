use tokio::net::{TcpListener};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        match listener.accept().await{
            Ok((mut socket, _addr)) =>  socket.write_all(b"ahah2").await.unwrap(),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}