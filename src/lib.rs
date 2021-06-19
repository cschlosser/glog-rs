use log::{Log, Record, Metadata, Level};
use std::cell::RefCell;
use termcolor::{StandardStream};
use thread_local::CachedThreadLocal;
use termcolor::{ColorSpec, ColorChoice, Color, WriteColor};
use std::io::{LineWriter, Write};
use std::path::Path;
use chrono::Local;
use std::convert::TryInto;
use backtrace::Backtrace;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::str::FromStr;
use if_empty::*;

mod flags;

pub use flags::Flags as Flags;

pub struct Glog {
    writer: CachedThreadLocal<RefCell<StandardStream>>,
    compatible_verbosity: bool,
    compatible_date: bool,
    flags: Flags,
    log_file_name: OsString,
}

impl Glog {
    pub fn new() -> Glog {
        Glog {
            writer: CachedThreadLocal::new(),
            compatible_verbosity: true,
            compatible_date: true,
            flags: Flags::default(),
            log_file_name: OsString::new(),
        }
    }
    pub fn init(&mut self, flags: Flags) -> Result<(), log::SetLoggerError> {
        self.flags = flags;
        if !self.flags.logtostderr {
            self.create_log_files();
        }
        // todo: restore this once this can be changed during runtime for glog
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

    fn create_log_files(&mut self) {
        let mut log_file_path = PathBuf::new();
        log_file_path.push(self.flags.log_dir.clone());
        let mut log_file_name = OsString::new();
        log_file_name.push(std::env::current_exe().unwrap_or(PathBuf::from_str("UNKNOWN").unwrap_or(PathBuf::new())).file_name().unwrap_or(OsStr::new("UNKNOWN")));
        log_file_name.push(".");
        log_file_name.push(gethostname::gethostname().if_empty(OsString::from("(unknown)")));
        log_file_name.push(".");
        let uname = whoami::username().if_empty("invalid-user".to_string());
        log_file_name.push(uname);
        log_file_name.push(".log.");

        let mut log_file_suffix = OsString::new();
        log_file_suffix.push(".");
        log_file_suffix.push(Local::now().format("%Y%m%d-%H%M%S").to_string());
        log_file_suffix.push(".");
        log_file_suffix.push(std::process::id().to_string());

        let levels = if self.compatible_verbosity { vec![Level::Info, Level::Warn, Level::Error] } else { vec![Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error] };
        for level in levels.iter() {
            println!("{}/{}{}{}", log_file_path.to_str().unwrap(), log_file_name.to_str().unwrap(), level.to_string().to_uppercase(), log_file_suffix.to_str().unwrap());
        }

    }

    fn should_log_backtrace(&self, file_name: &str, line: u32) -> bool {
        if self.flags.log_backtrace_at.is_some() {
            format!("{}:{}", file_name, line) == *self.flags.log_backtrace_at.as_ref().unwrap()
        } else {
            false
        }
    }

    fn build_log_message(&self, record: &Record, file_name: &str) -> String {
        format!("{}{} {:5} {}:{}] {}",
            match record.metadata().level() {
                Level::Error => "E",
                Level::Warn => "W",
                Level::Debug if self.compatible_verbosity == false => "D",
                Level::Trace if self.compatible_verbosity == false => "T",
                _ => "I",
            },
            Local::now().format(
                &format!("{}%m%d %H:%M:%S%.6f",
                    if self.compatible_date { "" } else { "%Y" }
                )
            ),
            get_tid(),
            file_name,
            record.line().unwrap(),
            record.args(),
        )
    }

    fn write_stderr(&self, record: &Record) {
        let writer = self.writer.get_or(|| RefCell::new(StandardStream::stderr(ColorChoice::Auto)));
        let writer = writer.borrow_mut();
        let mut writer = LineWriter::new(writer.lock());

        if self.flags.colorlogtostderr {
            writer.get_mut().set_color(ColorSpec::new().set_fg(match record.metadata().level() {
                Level::Error => Some(Color::Red),
                Level::Warn => Some(Color::Yellow),
                _ => None
            })).expect("failed to set color");
        }

        let file_name = Path::new(record.file().unwrap_or("")).file_name().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap_or("");

        writeln!(writer, "{}", self.build_log_message(record, file_name)).expect("couldn't write log message");

        if self.flags.colorlogtostderr {
            writer.get_mut().reset().expect("failed to reset color");
        }

        if self.should_log_backtrace(file_name, record.line().unwrap_or(0)) {
            writeln!(writer, "{:?}", Backtrace::new()).expect("Couldn't write backtrace");
        }
    }

    fn write_file(&self, record: &Record) {
    }

    fn write_sinks(&self) {
    
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
fn get_tid() -> u64 {
    bindings::Windows::Win32::System::Threading::GetCurrentThreadId().try_into().unwrap()
}

impl Log for Glog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.flags.minloglevel >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return
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
        let writer = self.writer.get_or(|| RefCell::new(StandardStream::stderr(ColorChoice::Auto)));
        let mut writer = writer.borrow_mut();
        writer.flush().ok();
    }
}

impl Clone for Glog {
    fn clone(&self) -> Glog {
        Glog {
            writer: CachedThreadLocal::new(),
            flags: self.flags.clone(),
            log_file_name: self.log_file_name.clone(),
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

