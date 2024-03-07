use iso8601_timestamp::Timestamp;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub enum LogLevel {
	Error,
	Warn,
	Info,
	Debug,
	Trace,
}

static LOG_LEVEL_NAMES: [&str; 5] = [
	"\x1B[31mERROR\x1B[0m",
	"\x1B[33mWARN \x1B[0m",
	"\x1B[36mINFO \x1B[0m",
	"\x1B[35mDEBUG\x1B[0m",
	"TRACE",
];

impl Display for LogLevel {
	fn fmt(&self, fmt: &mut Formatter) -> Result {
		fmt.pad(LOG_LEVEL_NAMES[*self as usize])
	}
}

pub fn log<Message: AsRef<str>>(level: LogLevel, message: Message) {
	println!(
		"{} {} [APP] {}",
		Timestamp::now_utc().format(),
		level,
		message.as_ref()
	);
}

#[allow(unused)]
pub(crate) fn core_log<Message: AsRef<str>>(level: LogLevel, message: Message) {
	println!(
		"{} {} [HAZEL] {}",
		Timestamp::now_utc().format(),
		level,
		message.as_ref()
	);
}

#[macro_export]
macro_rules! trace {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::log($crate::LogLevel::Trace, format!($($args),+))
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
            $crate::log($crate::LogLevel::Debug, format!($($args),+))
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
            $crate::log($crate::LogLevel::Info, format!($($args),+))
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
            $crate::log($crate::LogLevel::Warn, format!($($args),+))
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
            $crate::log($crate::LogLevel::Error, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_trace {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::core_log($crate::LogLevel::Trace, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_debug {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::core_log($crate::LogLevel::Debug, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_info {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::core_log($crate::LogLevel::Info, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_warn {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::core_log($crate::LogLevel::Warn, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
macro_rules! core_error {
    ($($args:expr),+) => {
        #[cfg(debug_assertions)]
        {
            $crate::core_log($crate::LogLevel::Error, format!($($args),+))
        }
        #[cfg(not(debug_assertions))]
        {
            ($($args),+)
        }
    };
}

#[allow(unused)]
pub(crate) use {core_debug, core_error, core_info, core_trace, core_warn};
