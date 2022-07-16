mod produci_consuma;

use std::{io::Result, sync::Condvar};
use produci_consuma::{Consumer, Controller, Buffo};
use std::sync::{Arc, Mutex, MutexGuard};

#[tokio::main]
async fn main() {
    let mut buffer = Buffo::new();
    let mut ctrl = Controller::new(
        Arc::new((Mutex::new(&buffer), Condvar::new()))
    );
    let cons = Consumer::new(&ctrl);

    ctrl.start();
    cons.start();
}
