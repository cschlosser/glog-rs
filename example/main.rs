use log::*;
use glog::Flags;

fn foo() {
    use std::thread;

let handler = thread::Builder::new()
    .name("named thread".into())
    .spawn(|| {
        let handle = thread::current();
        info!("from other thread?");
        debug!("a debug msg");
        trace!("some tracing");
        assert_eq!(handle.name(), Some("named thread"));
    })
    .unwrap();

handler.join().unwrap();

    warn!("a warning from foo()");
}

fn main() {
    glog::new()
        .with_year(true)
        .limited_abbreviations(true)
        .init(Flags {
            colorlogtostderr: true,
            minloglevel: Level::Trace,
        })
        .unwrap();

    error!("some failure in main while testing the logger");

    foo();    
}

