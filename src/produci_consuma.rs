use std::{sync::{Arc, Condvar, Mutex}, ops::DerefMut};

use tokio::net::TcpListener;

pub struct Consumer<'a> {
    ctrl: &'a Controller,
}

impl<'a> Consumer<'a> {
    pub fn new(ctrl: &'a Controller) -> Consumer<'a> {
        Consumer { ctrl }
    }

    pub fn start(&self) {
        let prod = self.ctrl.get_buffer().clone();
        tokio::spawn(move || {
            let &(ref lock, ref cvar) = &*prod;
            let mut fetched = lock.lock().unwrap();
            loop {
                fetched = cvar.wait(fetched).unwrap();
                println!("Recieved {}", name);
            }
        });
    }
}

pub struct Controller {
    buffer: Arc<(Mutex<[u8;16]>, Condvar)>,
}

impl Controller {
    pub fn new(buffer: Arc<(Mutex<[u8;16]>, Condvar)>) -> Controller {
        Controller { buffer }
    }

    pub fn get_buffer(&self) -> &Arc<(Mutex<[u8;16]>, Condvar)> {
        &self.buffer
    }

    pub fn start(&self) {
        tokio::spawn(async move {
            let bound = TcpListener::bind("127.0.0.1:6379");
            let listener = bound.await.unwrap();
            let connection = listener.accept().await;
            loop {
                println!("===LOOP===");
                match connection {
                    Ok((stream, addr)) => {
                        println!("New client: {:?}", addr);
                        let &(lock, cvar) = &*self.buffer;  
                        let b = *lock.lock().unwrap();
                        let c = &mut [b[0]];
                        match stream.try_read(c) {
                            Ok(n) => println!("Read {} bytes", n),
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            }
        });
    }
}
