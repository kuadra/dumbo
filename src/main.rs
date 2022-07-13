mod produci_consuma;

use std::{io::Result, sync::Condvar};
use produci_consuma::{Consumer, Controller};
use std::sync::{Arc, Mutex, MutexGuard};

#[tokio::main]
async fn main() {
    let ctrl = Controller::new(
        Arc::new((Mutex::new(false), Condvar::new()))
    );
    let cons = Consumer::new(&ctrl);

    ctrl.start();
    cons.start();
}
