use std::error::Error;
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

#[tokio::main]
 async fn main() {
    match TcpListener::bind("127.0.0.1:6379").await {
        Ok(res) => handle_connection(res).await,
        Err(err) => println!("Binding error")
    }
}

async fn handle_connection(listener : TcpListener) {
    println!("{:?}",listener);
    loop{
        match listener.accept().await {
            Ok(res) => println!("ohohoho")  ,
            Err(err) => println!("Binding error")  
        }
    }
}
