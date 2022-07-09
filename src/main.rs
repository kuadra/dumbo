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
    
    stream.writable().await.unwrap();
    match stream.try_write(b"asd\n") {
        Ok(n) => println!("Wrote {} bytes", n),
        Err(e) => println!("{:?}", e),
    }
    stream.readable().await.unwrap();
    let mut a = [0;10];
    match stream.try_read(&mut a) {
        Ok(n) => println!("Read {} bytes", n),
        Err(e) => println!("{:?}", e),
    }
    println!("Buffer A: {:?}", a);
}
