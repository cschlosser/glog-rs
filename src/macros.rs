///
/// Log a message at the Fatal level and then panic!
///
/// This will always panic. Regardless if `glog` is initialzed or not
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {{
        let logger = $crate::GLOG.logger.read().unwrap();
        let file_path = std::path::Path::new(file!());
        let file_name = file_path.file_name().unwrap_or_default().to_str().unwrap_or_default();
        let record = $crate::Record {
            level: glog::Level::Fatal,
            args: &__log_format_args!($($arg)+),
            file: file_name,
            line: line!(),
        };

        logger.log_internal(&record);
        if logger.initialized {
            panic!()
        } else {
            panic!($($arg)+)
        }
    }};
}
