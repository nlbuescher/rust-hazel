mod event;

use colored::Colorize;
use event::{Event, EventCategory};
use iso8601_timestamp::Timestamp;
use simple_logger::SimpleLogger;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

static LOG_LEVEL_NAMES: [&str; 5] = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(LOG_LEVEL_NAMES[*self as usize])
    }
}

pub fn log<Message: AsRef<str>>(level: Level, message: Message) {
    let level_string: String = match level {
        Level::Error => format!("{:<5}", level.to_string().red()),
        Level::Warn => format!("{:<5}", level.to_string().yellow()),
        Level::Info => format!("{:<5}", level.to_string().cyan()),
        Level::Debug => format!("{:<5}", level.to_string().purple()),
        Level::Trace => format!("{:<5}", level.to_string().normal()),
    };
    println!(
        "{} {} [APP] {}",
        Timestamp::now_utc().format(),
        level_string,
        message.as_ref()
    );
}

fn core_log<Message: AsRef<str>>(level: Level, message: Message) {
    let level_string: String = match level {
        Level::Error => format!("{:<5}", level.to_string().red()),
        Level::Warn => format!("{:<5}", level.to_string().yellow()),
        Level::Info => format!("{:<5}", level.to_string().cyan()),
        Level::Debug => format!("{:<5}", level.to_string().purple()),
        Level::Trace => format!("{:<5}", level.to_string().normal()),
    };
    println!(
        "{} {} [HAZEL] {}",
        Timestamp::now_utc().format(),
        level_string,
        message.as_ref()
    );
}

#[macro_export]
macro_rules! trace {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Trace, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Debug, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Info, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Warn, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            log(Level::Error, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

macro_rules! core_trace {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            core_log(Level::Trace, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

macro_rules! core_debug {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            core_log(Level::Debug, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

macro_rules! core_info {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            core_log(Level::Info, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

macro_rules! core_warn {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            crate::core_log(Level::Warn, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

macro_rules! core_error {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            hazel::core_log(Level::Error, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

pub fn run<T: Application>(create_application: impl Fn() -> T) {
    SimpleLogger::new().init().unwrap();
    core_warn!("Initialized Log!");
    let a = 5;
    info!("Hello! Var={a}");
    
    let application = create_application();
    application.run();
}

pub trait Application {
    fn run(&self) {
        let event = Event::WindowResize { width: 1280, height: 720 };
        if event.is_in_category(EventCategory::Application) {
            trace!("{event}");
        }
        if event.is_in_category(EventCategory::Input) {
            trace!("{event}");
        }
        loop {}
    }
}
