use log::{Log, Record, Metadata, LevelFilter, Level};
use std::cell::RefCell;
use termcolor::{StandardStream};
use thread_local::CachedThreadLocal;
use termcolor::{ColorSpec, ColorChoice, Color, WriteColor};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Flags {
    pub colorlogtostderr: bool,
    pub minloglevel: Level,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            colorlogtostderr: false,
            minloglevel: Level::Info,
        }
    }
}

pub struct GLog {
    writer: CachedThreadLocal<RefCell<StandardStream>>,
    compatible_verbosity: bool,
    compatible_date: bool,
    flags: Flags,
}

impl GLog {
    pub fn new() -> GLog {
        GLog {
            writer: CachedThreadLocal::new(),
            compatible_verbosity: true,
            compatible_date: true,
            flags: Flags::default(),
        }
    }
    pub fn init(&mut self, flags: Flags) -> Result<(), log::SetLoggerError> {
        self.flags = flags;
        log::set_max_level(LevelFilter::Trace);
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

impl Log for GLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.flags.minloglevel >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return
        }
        let writer = self
            .writer
            .get_or(|| RefCell::new(StandardStream::stderr(ColorChoice::Auto)));
        let writer = writer.borrow_mut();
        let mut writer = io::LineWriter::new(writer.lock());
        if self.flags.colorlogtostderr {
            writer.get_mut().set_color(ColorSpec::new().set_fg(match record.metadata().level() {
                Level::Error => Some(Color::Red),
                Level::Warn => Some(Color::Yellow),
                _ => None
            })).expect("failed to set color");
        }
        let file_name = Path::new(record.file().unwrap()).file_name().unwrap().to_str().unwrap();
        let _ = writeln!(writer, "{}{} {} {}:{}] {}",
                    match record.metadata().level() {
                        Level::Error => "E",
                        Level::Warn => "W",
                        Level::Debug if self.compatible_verbosity == false => "D",
                        Level::Trace if self.compatible_verbosity == false => "T",
                        _ => "I",
                    },
                    Local::now().format(&format!("{}%m%d %H:%M:%S%.6f", if self.compatible_date { "" } else { "%Y" })),
                    get_tid(),
                    file_name,
                    record.line().unwrap(),
                    record.args(),
        );
        if self.flags.colorlogtostderr {
            writer.get_mut().reset().expect("failed to reset color");
        }
    }

    fn flush(&self) {
        let writer = self
            .writer
            .get_or(|| RefCell::new(StandardStream::stderr(ColorChoice::Auto)));
        let mut writer = writer.borrow_mut();
        writer.flush().ok();
    }
}

impl Clone for GLog {
    fn clone(&self) -> GLog {
        GLog {
            writer: CachedThreadLocal::new(),
            flags: self.flags.clone(),
            ..*self
        }
    }
}

impl Default for GLog {
    fn default() -> Self {
        GLog::new()
    }
}

pub fn new() -> GLog {
    GLog::new()
}

