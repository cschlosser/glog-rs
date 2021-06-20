use std::{env::temp_dir, ffi::OsString, path::PathBuf};

use log::Level;

#[derive(Debug, Clone)]
pub struct Flags {
    pub colorlogtostderr: bool,
    pub minloglevel: Level,
    pub log_backtrace_at: Option<String>,
    pub logtostderr: bool,
    pub alsologtostderr: bool,
    pub log_dir: OsString,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            colorlogtostderr: false,
            minloglevel: Level::Info,
            log_backtrace_at: None,
            logtostderr: false,
            alsologtostderr: false,
            log_dir: [
                temp_dir().into_os_string(),
                OsString::from(""), // Users may not append a / or \ to their env vars
            ]
            .iter()
            .collect::<PathBuf>()
            .into_os_string(),
        }
    }
}
