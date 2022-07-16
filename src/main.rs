use tokio::sync::mpsc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let t1 = tokio::spawn(async move {
        let bound = TcpListener::bind("127.0.0.1:6379");
        let listener = bound.await.unwrap();
        loop {
            println!("===LOOP===");
            let a = listener.accept();
            let b = a.await;
            match b {
                Ok((stream, addr)) => {
                    println!("New client: {:?}", addr);
                    let mut b: [u8; 32] = [0; 32];
                    stream.readable().await.unwrap();
                    match stream.try_read(&mut b) {
                        Ok(n) => println!("Read {} bytes", n),
                        Err(e) => println!("Error: {:?}", e),
                    }
                    tx.send(b).await.unwrap();
                }
                Err(err) => println!("{}", err),
            }
        }
    });

    let t2 = tokio::spawn(async move {
        loop {
            let a = rx.recv().await;
            match a {
                Some(res) => println!("T2 Received: {:?}", res),
                None => println!("Porcodio"),
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
