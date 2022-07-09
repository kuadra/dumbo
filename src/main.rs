use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};


#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        println!("===LOOP===");        
        let a = listener.accept();
        let b = a.await;
        match b {
            Ok(res) => {
                //println!("{:?}", res.0);
                println!("New client: {:?}", res.1);
                handle(res.0).await;
                res.0.writable().await;
                res.0.try_write(b"asd");
            }
            Err(err) => println!("{}", err),
        }
    }
}

async fn handle(mut stream : TcpStream) {
    let a = &mut String::new();
    stream.read_to_string(a).await.unwrap();
    println!("{}",a);
}
