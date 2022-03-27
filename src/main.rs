use tokio::net::TcpListener;

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let a = listener.accept();
        let b = a.await;
        match b {
            Ok(res) => {
                println!("{:?}", res);
                println!("new client: {:?}", res.1)
            }
            Err(err) => println!("{}", err),
        }
    }
}
