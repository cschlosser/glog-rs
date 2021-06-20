use std::{env::temp_dir, ffi::OsString, path::PathBuf};

use log::Level;

/// The flag structure used to initialize glog.
///
/// The flags have the same name and defaults as in [`glog`] but use Rust types where possible.
///
/// [`glog`]: https://github.com/google/glog
///
/// # Example
///
/// This are the defaults for each flag:
/// ```
/// use std::{env::temp_dir, ffi::OsString, path::PathBuf};
/// use log::*;
/// use glog::Flags;
///
/// let flags = Flags::default();
///
/// assert_eq!(flags.colorlogtostderr, false);
/// assert_eq!(flags.minloglevel, Level::Info);
/// assert!(flags.log_backtrace_at.is_none());
/// assert_eq!(flags.logtostderr, false);
/// assert_eq!(flags.alsologtostderr, false);
/// assert_eq!(flags.log_dir, [temp_dir(), PathBuf::from("")].iter().collect::<PathBuf>().into_os_string());
/// ```
#[derive(Debug, Clone)]
pub struct Flags {
    /// [`Info`]: ../log/enum.Level.html#variant.Info
    /// If logging to stderr try to colorize levels more severe than [`Info`]
    pub colorlogtostderr: bool,
    /// Minimum level (inclusive) that should be logged
    pub minloglevel: Level,
    /// Optionally log a backtrace at `filename:line` log invocation.
    /// The log level has to be enabled for it to work.
    /// Will be written in the log file with the lowest severity.
    pub log_backtrace_at: Option<String>,
    /// Log to stderr instead of logfiles
    pub logtostderr: bool,
    /// Log to stderr and logfiles
    pub alsologtostderr: bool,
    /// Directory in which to store the log files
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
