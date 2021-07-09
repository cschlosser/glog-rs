use std::thread;

use glog::{fatal, Extensions, Flags};
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
    glog::init(Flags {
        colorlogtostderr: true,
        minloglevel: Level::Trace,
        log_backtrace_at: Some("main.rs:20".to_owned()),
        alsologtostderr: true,
        ..Default::default()
    })
    .unwrap();

    error!("some error in main while testing the logger");

    foo();

    info!("Still default");
    glog::Glogger::enable_extensions(Extensions {
        with_year: true,
        ..Default::default()
    });
    debug!("With year");
    glog::Glogger::with_year(false);
    glog::Glogger::reduced_log_levels(false);
    trace!("Without year again but with additional severity levels now");

    fatal!("This will stop the application");
}
