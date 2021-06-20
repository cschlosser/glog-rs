//!
//! Port of the famous C++ [`glog`] logging style framework.
//!
//! It's implemented on top of the [`standard logging`] crate in Rust.
//!
//! Default options will be compatible with [`glog`] but there are several customizations possible
//! to take advantage of more Rust standard features (currently more levels) and additional configurability options.
//!
//! [`glog`]: https://github.com/google/glog
//! [`standard logging`]: https://crates.io/crates/log
//!
//! ## Examples
//!
//! ### Basic usage
//!
//! ```
//! use log::*;
//! use glog::Flags;
//!
//! glog::new().init(Flags::default()).unwrap();
//!
//! info!("A log message");
//! ```
//!
//! ### Pretty logs on stderr
//!
//! By default glog will write to files once initialized.
//! For colored priniting to stderr you can use these flags:
//!
//! ```
//! use log::*;
//! use glog::Flags;
//!
//! glog::new().init(Flags {
//!         colorlogtostderr: true,
//!         alsologtostderr: true, // use logtostderr to only write to stderr and not to files
//!         ..Default::default(),
//!     }).unwrap();
//!
//! info!("This will be visibile on stderr and in a file");
//! // I0401 12:34:56.987654   123 doc.rs:9]
//! ```
//!
//! ### Nonstandard Glog configuration
//!
//! [`glog`] doesn't have levels for Trace and Debug. Just like Verbose logs in [`glog`] these will
//! be logged as INFO by default.
//! As an additional configuration this logging crate offers these levels as different ones as
//! well.
//!
//! ```
//! use log::*;
//! use glog::Flags;
//!
//! glog::new()
//!     .limited_abbreviations(false) // Treat DEBUG and TRACE as separate levels
//!     .with_year(true) // Add the year to the timestamp in the logfile
//!     .init(Flags {
//!         minloglevel: Level::Trace, // By default glog will only log INFO and more severe
//!         logtostderr: true, // don't write to log files
//!         ..Default::default(),
//!     }).unwrap();
//!
//! trace!("A trace message");
//! debug!("Helpful for debugging");
//! info!("An informational message");
//!
//! // T20210401 12:34:56.000000  1234 doc.rs:14] A trace message
//! // D20210401 12:34:56.000050  1234 doc.rs:15] Helpful for debugging
//! // I20210401 12:34:56.000100  1234 doc.rs:16] An informational message
//! ```

use std::{
    cell::RefCell,
    collections::HashMap,
    convert::TryInto,
    ffi::{OsStr, OsString},
    fs::{File, OpenOptions},
    io::{LineWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Arc, Mutex},
};

use backtrace::Backtrace;
use bimap::BiMap;
use chrono::{DateTime, Local};
use if_empty::*;
use log::{Level, Log, Metadata, Record};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use thread_local::CachedThreadLocal;

mod flags;

pub use flags::Flags;

pub struct Glog {
    stderr_writer: CachedThreadLocal<RefCell<StandardStream>>,
    compatible_verbosity: bool,
    compatible_date: bool,
    flags: Flags,
    application_fingerprint: Option<String>,
    start_time: DateTime<Local>,
    file_writer: HashMap<Level, Arc<Mutex<RefCell<File>>>>,
    level_integers: BiMap<Level, i8>,
}

impl Glog {
    pub fn new() -> Glog {
        Glog {
            stderr_writer: CachedThreadLocal::new(),
            compatible_verbosity: true,
            compatible_date: true,
            flags: Flags::default(),
            application_fingerprint: None,
            start_time: Local::now(),
            file_writer: HashMap::new(),
            level_integers: BiMap::new(),
        }
    }

    pub fn init(&mut self, flags: Flags) -> Result<(), log::SetLoggerError> {
        self.level_integers.insert(Level::Trace, -2);
        self.level_integers.insert(Level::Debug, -1);
        self.level_integers.insert(Level::Info, 0);
        self.level_integers.insert(Level::Warn, 1);
        self.level_integers.insert(Level::Error, 2);
        self.flags = flags;
        if !self.flags.logtostderr {
            self.create_log_files();
        }
        // todo(#4): restore this once this can be changed during runtime for glog
        // log::set_max_level(LevelFilter::Trace);
        log::set_max_level(self.flags.minloglevel.to_level_filter());
        log::set_boxed_logger(Box::new(self.clone()))
    }

    pub fn with_year(mut self, with_year: bool) -> Self {
        self.compatible_date = !with_year;
        self
    }

    pub fn limited_abbreviations(mut self, limit_abbreviations: bool) -> Self {
        self.compatible_verbosity = limit_abbreviations;
        self
    }

    pub fn set_application_fingerprint(mut self, fingerprint: &str) -> Self {
        self.application_fingerprint = Some(fingerprint.to_owned());
        self
    }

    fn match_level(&self, level: &Level) -> Level {
        match level {
            Level::Debug if self.compatible_verbosity => Level::Info,
            Level::Trace if self.compatible_verbosity => Level::Info,
            _ => *level,
        }
    }

    fn create_log_files(&mut self) {
        let log_file_dir = self.flags.log_dir.clone();
        let mut log_file_name = OsString::new();
        log_file_name.push(
            std::env::current_exe()
                .unwrap_or_else(|_| PathBuf::from_str("UNKNOWN").unwrap_or_default())
                .file_name()
                .unwrap_or_else(|| OsStr::new("UNKNOWN")),
        );
        log_file_name.push(".");
        log_file_name.push(gethostname::gethostname().if_empty(OsString::from("(unknown)")));
        log_file_name.push(".");
        log_file_name.push(whoami::username().if_empty("invalid-user".to_string()));
        log_file_name.push(".log.");

        let log_file_suffix = format!(
            ".{}.{}",
            Local::now().format("%Y%m%d-%H%M%S").to_string(),
            std::process::id().to_string()
        );

        let mut log_file_base = OsString::new();
        log_file_base.push(log_file_dir);
        log_file_base.push(log_file_name);
        if !self.compatible_verbosity {
            for level in &[Level::Trace, Level::Debug] {
                let mut log_file_path = log_file_base.clone();
                log_file_path.push(level.to_string().to_uppercase());
                log_file_path.push(log_file_suffix.to_string());
                self.write_file_header(&log_file_path, level);
            }
        }
        for level in &[Level::Info, Level::Warn, Level::Error] {
            let mut log_file_path = log_file_base.clone();
            log_file_path.push(level.to_string().to_uppercase());
            log_file_path.push(log_file_suffix.to_string());
            self.write_file_header(&log_file_path, level);
        }
    }

    fn write_file_header(&mut self, file_path: &OsString, level: &Level) {
        {
            let mut file = match File::create(&file_path) {
                Err(why) => panic!(
                    "couldn't create {}: {}",
                    file_path.to_str().unwrap_or("<INVALID FILE PATH>"),
                    why
                ),
                Ok(file) => file,
            };

            let running_duration = Local::now() - self.start_time;

            // todo(#3): integrate UTC
            file.write_fmt(
                format_args!("Log file created at:\n{}\nRunning on machine: {}\n{}Running duration (h:mm:ss): {}:{:02}:{:02}\nLog line format: [{}IWE]{}mmdd hh:mm:ss.uuuuuu threadid file:line] msg\n",
                    Local::now().format("%Y/%m/%d %H:%M:%S"),
                    gethostname::gethostname().to_str().unwrap_or("UNKNOWN"),
                    if self.application_fingerprint.is_some() { format!("Application fingerprint: {}\n", self.application_fingerprint.clone().unwrap()) } else { String::new() },
                    running_duration.num_hours(),
                    running_duration.num_minutes(),
                    running_duration.num_seconds(),
                    if self.compatible_verbosity { "" } else { "TD" },
                    if self.compatible_date { "" } else { "yyyy" },
                )
            ).expect("couldn't write log file header");

            if let Err(why) = file.flush() {
                panic!(
                    "couldn't flush {} after writing file header: {}",
                    file_path.to_str().unwrap(),
                    why
                )
            }
        }
        self.file_writer.insert(
            *level,
            Arc::new(Mutex::new(RefCell::new(
                OpenOptions::new()
                    .append(true)
                    .open(&file_path)
                    .expect("Couldn't open file after header is written"),
            ))),
        );
    }

    fn should_log_backtrace(&self, file_name: &str, line: u32) -> bool {
        if self.flags.log_backtrace_at.is_some() {
            format!("{}:{}", file_name, line) == *self.flags.log_backtrace_at.as_ref().unwrap()
        } else {
            false
        }
    }

    fn record_to_file_name(record: &Record) -> String {
        Path::new(record.file().unwrap_or(""))
            .file_name()
            .unwrap_or_default()
            .to_os_string()
            .into_string()
            .unwrap_or_default()
    }

    fn build_log_message(&self, record: &Record) -> String {
        format!(
            "{}{} {:5} {}:{}] {}",
            self.match_level(&record.metadata().level()).as_str().chars().next().unwrap(),
            Local::now().format(&format!("{}%m%d %H:%M:%S%.6f", if self.compatible_date { "" } else { "%Y" })),
            get_tid(),
            Glog::record_to_file_name(record),
            record.line().unwrap(),
            record.args(),
        )
    }

    fn write_stderr(&self, record: &Record) {
        let stderr_writer = self
            .stderr_writer
            .get_or(|| RefCell::new(StandardStream::stderr(ColorChoice::Auto)));
        let stderr_writer = stderr_writer.borrow_mut();
        let mut stderr_writer = LineWriter::new(stderr_writer.lock());

        if self.flags.colorlogtostderr {
            stderr_writer
                .get_mut()
                .set_color(ColorSpec::new().set_fg(match record.metadata().level() {
                    Level::Error => Some(Color::Red),
                    Level::Warn => Some(Color::Yellow),
                    _ => None,
                }))
                .expect("failed to set color");
        }

        let file_name = Glog::record_to_file_name(record);

        writeln!(stderr_writer, "{}", self.build_log_message(record)).expect("couldn't write log message");

        if self.flags.colorlogtostderr {
            stderr_writer.get_mut().reset().expect("failed to reset color");
        }

        if self.should_log_backtrace(&file_name, record.line().unwrap_or(0)) {
            writeln!(stderr_writer, "{:?}", Backtrace::new()).expect("Couldn't write backtrace");
        }
    }

    fn level_as_int(&self, level: &Level) -> i8 {
        *self.level_integers.get_by_left(&self.match_level(level)).unwrap()
    }

    fn write_file(&self, record: &Record) {
        // prevent writing to non existing writer if minloglevel is <INFO
        for level_int in self.level_as_int(&self.flags.minloglevel)..=self.level_as_int(&record.level()) {
            let level = self.level_integers.get_by_right(&level_int).unwrap();
            let file_write_guard = self.file_writer.get(level).unwrap().lock().unwrap();
            let mut file_writer = (*file_write_guard).borrow_mut();
            if let Err(why) = file_writer.write_fmt(format_args!("{}\n", self.build_log_message(record))) {
                panic!("couldn't write log message to file for level {}: {}", record.level(), why)
            }
        }

        if self.should_log_backtrace(&Glog::record_to_file_name(record), record.line().unwrap_or(0)) {
            let level = self.match_level(&self.flags.minloglevel);
            let file_write_guard = self.file_writer.get(&level).unwrap().lock().unwrap();
            let mut file_writer = (*file_write_guard).borrow_mut();
            if let Err(why) = file_writer.write_fmt(format_args!("{:?}\n", Backtrace::new())) {
                panic!("couldn't write backtrace to {} file: {}", level, why)
            }
        }
    }

    fn write_sinks(&self) {}
}

impl Log for Glog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.flags.minloglevel >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        if self.flags.logtostderr || self.flags.alsologtostderr {
            self.write_stderr(record);
        }
        if !self.flags.logtostderr {
            self.write_file(record);
        }
        self.write_sinks();
    }

    fn flush(&self) {
        let stderr_writer = self
            .stderr_writer
            .get_or(|| RefCell::new(StandardStream::stderr(ColorChoice::Auto)));
        let mut stderr_writer = stderr_writer.borrow_mut();
        stderr_writer.flush().ok();

        for file in self.file_writer.values() {
            let file_guard = file.lock().unwrap();
            let mut file_writer = (*file_guard).borrow_mut();
            file_writer.flush().expect("couldn't sync log to disk");
        }
    }
}

#[cfg(target_os = "macos")]
fn get_tid() -> u64 {
    nix::sys::pthread::pthread_self().try_into().unwrap()
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn get_tid() -> u64 {
    nix::unistd::gettid().as_raw().try_into().unwrap()
}

#[cfg(target_os = "windows")]
mod bindings {
    windows::include_bindings!();
}
#[cfg(target_os = "windows")]
fn get_tid() -> u64 {
    use bindings::Windows::Win32::System::Threading;
    GetCurrentThreadId().try_into().unwrap()
}

impl Clone for Glog {
    fn clone(&self) -> Glog {
        Glog {
            stderr_writer: CachedThreadLocal::new(),
            flags: self.flags.clone(),
            application_fingerprint: self.application_fingerprint.clone(),
            file_writer: self.file_writer.clone(),
            level_integers: self.level_integers.clone(),
            ..*self
        }
    }
}

impl Default for Glog {
    fn default() -> Self {
        Glog::new()
    }
}

pub fn new() -> Glog {
    Glog::new()
}

#[cfg(test)]
mod tests {
    // todo(#6): Fill with tests
}
