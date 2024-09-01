#![allow(unused)]

use std::{
	fmt::{self, Display},
	time::{SystemTime, UNIX_EPOCH},
};

#[derive(Copy, Clone)]
pub enum Level {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}

static LOG_LEVEL_NAMES: [&str; 5] = [
	"TRACE",
	"\x1B[36mDEBUG\x1B[0m",
	"\x1B[32mINFO\x1B[0m",
	"\x1B[33mWARN\x1B[0m",
	"\x1B[31mERROR\x1B[0m",
];

impl Display for Level {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.pad(LOG_LEVEL_NAMES[*self as usize]) }
}

pub(crate) fn core_log(level: Level, message: impl Display) {
	let timestamp = chrono::Local::now().format("%F %T%.3f");
	println!("{timestamp} {level:<5} [HAZEL] {message}");
}

macro_rules! core_trace {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Trace, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Trace, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			core_trace!($rest, +);
		}
	}
}

pub(crate) use core_trace;

macro_rules! core_debug {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Debug, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Debug, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			core_debug!($rest, +);
		}
	}
}

pub(crate) use core_debug;

macro_rules! core_info {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Info, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Info, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			core_info!($rest, +);
		}
	}
}

pub(crate) use core_info;

macro_rules! core_warn {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Warn, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Warn, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			core_warn!($rest, +);
		}
	}
}

pub(crate) use core_warn;

macro_rules! core_error {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Error, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::core_log($crate::log::Level::Error, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			core_error!($rest, +);
		}
	}
}

pub(crate) use core_error;

pub fn log(level: Level, message: impl Display) {
	let timestamp = chrono::Local::now().format("%F %T%.3f");
	println!("{timestamp} {level:<5} [APP] {message}");
}

#[macro_export]
macro_rules! trace {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Trace, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Trace, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			trace!($rest, +);
		}
	}
}

pub(crate) use crate::trace;

#[macro_export]
macro_rules! debug {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Debug, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Debug, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			debug!($rest, +);
		}
	}
}

pub(crate) use crate::debug;

#[macro_export]
macro_rules! info {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Info, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Info, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			info!($rest, +);
		}
	}
}

pub(crate) use crate::info;

#[macro_export]
macro_rules! warn {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Warn, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Warn, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			warn!($rest, +);
		}
	}
}

pub(crate) use crate::warn;

#[macro_export]
macro_rules! error {
	($first:expr) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Error, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
		}
	};
	
	($first:expr $(, $rest: expr)+) => {
		if cfg!(debug_assertions) {
			$crate::log::log($crate::log::Level::Error, format!($first));
		}
		else {
			#[allow(dropping_references)]
			std::mem::drop($first);
			error!($rest, +);
		}
	}
}

pub(crate) use crate::error;
