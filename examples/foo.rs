use std::thread;
use log::*;

pub fn foo() {
    let handler = thread::Builder::new()
    .name("named thread".into())
    .spawn(|| {
        let handle = thread::current();
        info!("from other thread!");
        debug!("a debug msg");
        trace!("some tracing");
        assert_eq!(handle.name(), Some("named thread"));
    })
    .unwrap();

    handler.join().unwrap();

    warn!("a warning from foo()");
}

