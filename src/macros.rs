#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        unsafe {
            match glog::LOGGER.get() {
                Some(logger) => {
                    let record = glog::Record {
                        level: glog::Level::Fatal,
                        args: &__log_format_args!($($arg)+),
                        file: "foo",
                        line: 0,
                    };
                    logger.log_internal(&record)
                }
                _ => (),
            }
        }
    }};
}
