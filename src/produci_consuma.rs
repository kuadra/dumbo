use std::{sync::{Arc, Condvar, Mutex}, ops::DerefMut, thread};

use tokio::net::TcpListener;

pub struct Consumer<'a> {
    ctrl: &'a Controller<'a> ,
}

impl<'a> Consumer<'a> {
    pub fn new(ctrl: &'a Controller<'a>) -> Consumer<'a> {
        Consumer { ctrl }
    }

    pub fn start(&self) {
        let (mtx , cvar) = **self.ctrl.get_buffer();

        thread::spawn(move || {
            let mut fetched = mtx.lock().unwrap();
            loop {
                fetched = cvar.wait(fetched).unwrap();
                println!("Recieved asdasd");
            };
        });
    }
}

pub struct Controller<'a>{
    buffer: Arc<(Mutex<Buffo<'a>>, Condvar)>,
}

impl<'a> Controller<'a> {
    pub fn new(buffer: Arc<(Mutex<Buffo<'a>>, Condvar)>) -> Controller<'a> {
        Controller { buffer }
    }

    pub fn get_buffer(&self) -> &Arc<(Mutex<Buffo<'a>>, Condvar)> {
        &self.buffer
    }

    pub fn start(self) {
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
                        let c = b.get_mem();
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

pub struct Buffo<'a>{
    mem : &'a [u8;10]
}

impl<'a> Buffo<'a> {
    pub fn new() -> Buffo<'a> {
        Buffo {
            mem : &[0;10]
        }
    }

    pub fn get_mem(&mut self) -> &'a [u8;10] {
        self.mem
    }

}
