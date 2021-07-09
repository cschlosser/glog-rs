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
    glog::init(
        Flags {
            colorlogtostderr: true,
            minloglevel: Level::Trace,
            log_backtrace_at: Some("main.rs:20".to_owned()),
            alsologtostderr: true,
            ..Default::default()
        },
        Some(Extensions {
            with_year: true,
            ..Default::default()
        }),
    )
    .unwrap();
    /*
    match glog::logger() {
        Some(logger) => logger
            .borrow_mut()
            .with_year(true)
            .reduced_log_levels(true)
            .set_application_fingerprint("Example"),
        None => todo!(),
    }; */
    error!("some erro in main while testing the logger");

    foo();

    info!("{:?}", Flags { ..Default::default() });
    fatal!("This will stop the application");
}
