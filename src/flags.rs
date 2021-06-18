use log::Level;

#[derive(Debug, Clone)]
pub struct Flags {
    pub colorlogtostderr: bool,
    pub minloglevel: Level,
    pub log_backtrace_at: Option<String>,
    pub logtostderr: bool,
    pub alsologtostderr: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            colorlogtostderr: false,
            minloglevel: Level::Info,
            log_backtrace_at: None,
            logtostderr: false,
            alsologtostderr: false,
        }
    }
}

