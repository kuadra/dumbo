mod buffo;

use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};


#[tokio::main]
async fn main() {
    let bound = TcpListener::bind("127.0.0.1:6379");
    let listener = bound.await.unwrap();

    loop {
        println!("===LOOP===");
        let a = listener.accept();
        let b = a.await;
        match b {
            Ok(res) => {
                println!("New client: {:?}", res.1);
                handle(res.0).await;
            }
            Err(err) => println!("{}", err),
        }
    }
}

async fn handle(stream: TcpStream) {
    stream.readable().await.unwrap();
    let mut buf : buffo::Buffo = buffo::Buffo::new();
    match stream.try_read(buf.get_mem()) {
        Ok(n) => println!("Read {} bytes", n),
        Err(e) => println!("Error: {:?}", e),
    }
    println!("Buffer A: {:?}", String::from_utf8(buf.get_mem().to_vec()));
}
