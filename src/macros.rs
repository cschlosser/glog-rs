#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        unsafe {
            match glog::LOGGER.get() {
                Some(logger) => {
                    let file_path = std::path::Path::new(file!());
                    let file_name = file_path
                            .file_name()
                            .unwrap_or_default().to_str().unwrap_or_default();
                    let record = glog::Record {
                        level: glog::Level::Fatal,
                        args: &__log_format_args!($($arg)+),
                        file: file_name,
                        line: line!(),
                    };
                    logger.log_internal(&record);
                    panic!()
                }
                _ => (),
            }
        }
    }};
}
