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

pub struct Extensions {
    /// Enable the year in the log timestamp
    ///
    /// By default the year is not part of the timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use log::*;
    /// use glog::Flags;
    ///
    /// // init of glog happens here in examples
    ///
    /// info!("A log message");
    /// ```
    ///
    /// ## With year
    /// ```
    /// # use log::*;
    /// # use glog::Flags;
    /// glog::new().with_year(true).init(Flags::default()).unwrap();
    /// // Will log:
    /// // I20210401 12:34:56.987654   123 doc.rs:4] A log message
    /// ```
    ///
    /// ## Without year
    /// ```
    /// # use log::*;
    /// # use glog::Flags;
    /// glog::new().with_year(false).init(Flags::default()).unwrap();
    /// // Will log:
    /// // I0401 12:34:56.987654   123 doc.rs:4] A log message
    /// ```
    pub with_year: bool,
    /// [`Trace`]: ../log/enum.Level.html#variant.Trace
    /// [`Debug`]: ../log/enum.Level.html#variant.Debug
    /// [`Info`]: ../log/enum.Level.html#variant.Info
    /// Change the behavior regarding [`Trace`] and [`Debug`] levels
    ///
    /// If `limit_abbreviations` is set to `false` [`Trace`] and [`Debug`] get their own
    /// levels. Otherwise they will be logged in the [`Info`] level.
    ///
    /// By default `reduced_log_levels` is true.
    ///
    /// # Examples
    ///
    /// ```
    /// use log::*;
    /// use glog::Flags;
    ///
    /// // glog init happens here
    ///
    /// trace!("A trace message");
    /// debug!("Helpful for debugging");
    /// info!("An informational message");
    /// ```
    ///
    /// ## With all abbreviations
    ///
    /// ```
    /// # use log::*;
    /// # use glog::Flags;
    /// glog::new()
    ///     .reduced_log_levels(false) // Treat DEBUG and TRACE as separate levels
    ///     .init(Flags {
    ///         minloglevel: Level::Trace, // By default glog will only log INFO and more severe
    ///         logtostderr: true, // don't write to log files
    ///         ..Default::default()
    ///     }).unwrap();
    ///
    /// // T0401 12:34:56.000000  1234 doc.rs:12] A trace message
    /// // D0401 12:34:56.000050  1234 doc.rs:13] Helpful for debugging
    /// // I0401 12:34:56.000100  1234 doc.rs:14] An informational message
    /// ```
    ///
    /// ## With limited abbreviations
    ///
    /// ```
    /// # use log::*;
    /// # use glog::Flags;
    /// glog::new()
    ///     .reduced_log_levels(true) // Treat DEBUG and TRACE are now logged as INFO
    ///     .init(Flags {
    ///         minloglevel: Level::Trace, // By default glog will only log INFO and more severe
    ///         logtostderr: true, // don't write to log files
    ///         ..Default::default()
    ///     }).unwrap();
    ///
    /// // I0401 12:34:56.000000  1234 doc.rs:12] A trace message
    /// // I0401 12:34:56.000050  1234 doc.rs:13] Helpful for debugging
    /// // I0401 12:34:56.000100  1234 doc.rs:14] An informational message
    /// ```
    pub with_rust_levels: bool,
}

impl Default for Extensions {
    fn default() -> Self {
        Extensions {
            with_year: false,
            with_rust_levels: false,
        }
    }
}
