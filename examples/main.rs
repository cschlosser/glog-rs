use std::thread;

use glog::Flags;
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

fn main() {
    glog::new()
        .with_year(true)
        .limited_abbreviations(true)
        .set_application_fingerprint("Example")
        .init(Flags {
            colorlogtostderr: true,
            minloglevel: Level::Trace,
            log_backtrace_at: Some("main.rs:20".to_owned()),
            alsologtostderr: true,
            ..Default::default()
        })
        .unwrap();

    error!("some erro in main while testing the logger");

    foo();

    info!(
        "{:?}",
        Flags {
            ..Default::default()
        }
    );
}
