use glog::Flags;
use log::*;

mod foo;

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

    foo::foo();
}
