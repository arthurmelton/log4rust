use colored::Color;
use lazy_static::lazy_static;
use std::io::{Error, ErrorKind};
use std::sync::{Mutex, MutexGuard, PoisonError};
use ureq::Request;

lazy_static! {
    #[doc(hidden)]
    pub static ref CONFIG: Mutex<Config> =  Mutex::new(new());
}

pub enum Log {
    Info,
    Warn,
    Error,
    Fatal,
    #[doc(hidden)]
    _None,
}

pub enum Backtrace {
    /// This will print no stack trace
    _None,
    /// This is going to just print the file, line number, and collom number
    Simple,
    /// This is going to print a full backtrace
    Complex,
}

#[derive(PartialEq)]
pub enum Time {
    UTC,
    Local,
}

/// To configure the logger you will use some different functions. <br>
/// This is how the default configuration would look if written out:
///
/// ```
/// log4rust::new()
///     .time(Time::Local)
///     .set_type(Log::Info)?.color(Color::TrueColor{r:0,g:255,b:255})?.console(true)?.backtrace(Backtrace::_None)?
///     .set_type(Log::Warn)?.color(Color::TrueColor{r:255,g:215,b:185})?.console(true)?.backtrace(Backtrace::_None)?
///     .set_type(Log::Error)?.color(Color::TrueColor{r:255,g:100,b:0})?.console(true)?.backtrace(Backtrace::Simple)?
///     .set_type(Log::Fatal)?.color(Color::TrueColor{r:255,g:0,b:0})?.console(true)?.backtrace(Backtrace::Complex)?
///     .save()
///     .unwrap();
/// ```
///
/// If you wanted to change something from the norm you could do
/// 
/// ```
/// log4rust::new()
///    .time(Time::UTC)
///    .set_type(Log::Info)?.console(false)?
///    .save()
///    .unwrap();
/// ```
/// You can see that we only change one of the items and all the rest are the same. We also change
/// it so that time will be saved in UTC and not in local time.
/// 
/// If you want to see some other examples look in
/// [https://github.com/AMTitan/log4rust/tree/master/examples](https://github.com/AMTitan/log4rust/tree/master/examples)
pub struct Config {
    // time
    #[doc(hidden)]
    pub time: Time,
    // colors
    #[doc(hidden)]
    pub info_color: Color,
    #[doc(hidden)]
    pub warn_color: Color,
    #[doc(hidden)]
    pub error_color: Color,
    #[doc(hidden)]
    pub fatal_color: Color,
    // console
    #[doc(hidden)]
    pub info_console: bool,
    #[doc(hidden)]
    pub warn_console: bool,
    #[doc(hidden)]
    pub error_console: bool,
    #[doc(hidden)]
    pub fatal_console: bool,
    // web
    #[doc(hidden)]
    pub info_web: Vec<(Request, String)>,
    #[doc(hidden)]
    pub warn_web: Vec<(Request, String)>,
    #[doc(hidden)]
    pub error_web: Vec<(Request, String)>,
    #[doc(hidden)]
    pub fatal_web: Vec<(Request, String)>,
    // files
    #[doc(hidden)]
    pub info_file: Vec<String>,
    #[doc(hidden)]
    pub warn_file: Vec<String>,
    #[doc(hidden)]
    pub error_file: Vec<String>,
    #[doc(hidden)]
    pub fatal_file: Vec<String>,
    // backtrace
    #[doc(hidden)]
    pub info_backtrace: Backtrace,
    #[doc(hidden)]
    pub warn_backtrace: Backtrace,
    #[doc(hidden)]
    pub error_backtrace: Backtrace,
    #[doc(hidden)]
    pub fatal_backtrace: Backtrace,
    // private
    working_on: Log,
}

pub fn new() -> Config {
    Config {
        time: Time::Local,
        info_color: Color::TrueColor {
            r: 0,
            g: 255,
            b: 255,
        },
        warn_color: Color::TrueColor {
            r: 255,
            g: 215,
            b: 185,
        },
        error_color: Color::TrueColor {
            r: 255,
            g: 100,
            b: 0,
        },
        fatal_color: Color::TrueColor { 
            r: 255, 
            g: 0, 
            b: 0
        },
        info_console: true,
        warn_console: true,
        error_console: true,
        fatal_console: true,
        info_web: Vec::new(),
        warn_web: Vec::new(),
        error_web: Vec::new(),
        fatal_web: Vec::new(),
        info_file: Vec::new(),
        warn_file: Vec::new(),
        error_file: Vec::new(),
        fatal_file: Vec::new(),
        info_backtrace: Backtrace::_None,
        warn_backtrace: Backtrace::_None,
        error_backtrace: Backtrace::Simple,
        fatal_backtrace: Backtrace::Complex,
        working_on: Log::_None,
    }
}

impl Config {
    /// This will change the type of time that will be saved (This will save it globally and not on
    /// a type by type basis)
    pub fn time(mut self, time: Time) -> Self {
        self.time = time;
        self
    }

    /// This will change the color of a type when it is printed to the console
    pub fn color(mut self, color: Color) -> Result<Self, Error> {
        match self.working_on {
            Log::Info => self.info_color = color,
            Log::Warn => self.warn_color = color,
            Log::Error => self.error_color = color,
            Log::Fatal => self.fatal_color = color,
            Log::_None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "You need to set a log type before you can set the color",
                ))
            }
        }
        Ok(self)
    }
    
    /// This will change the backtrace of a type when it is printed to the console
    pub fn backtrace(mut self, backtrace: Backtrace) -> Result<Self, Error> {
        match self.working_on {
            Log::Info => self.info_backtrace = backtrace,
            Log::Warn => self.warn_backtrace = backtrace,
            Log::Error => self.error_backtrace = backtrace,
            Log::Fatal => self.fatal_backtrace = backtrace,
            Log::_None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "You need to set a log type before you can set the backtrace",
                ))
            }
        }
        Ok(self)
    }

    /// This will set if this type will be printed to the console or not
    pub fn console(mut self, _bool: bool) -> Result<Self, Error> {
        match self.working_on {
            Log::Info => self.info_console = _bool,
            Log::Warn => self.warn_console = _bool,
            Log::Error => self.error_console = _bool,
            Log::Fatal => self.fatal_console = _bool,
            Log::_None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "You need to set a log type before you can set if it goes to the terminal",
                ))
            }
        }
        Ok(self)
    }

    /// This will make it so that you can make a request every time this type goes off. You can
    /// also have multiple of these so it will make multiple requests for every time this type
    /// fires. This would be most useful for a webhook.
    pub fn web(mut self, format: &str, request: Request) -> Result<Self, Error> {
        match self.working_on {
            Log::Info => self.info_web.push((request, format.to_string())),
            Log::Warn => self.warn_web.push((request, format.to_string())),
            Log::Error => self.error_web.push((request, format.to_string())),
            Log::Fatal => self.fatal_web.push((request, format.to_string())),
            Log::_None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "You need to set a log type before you can set where it should be sent to",
                ))
            }
        }
        Ok(self)
    }

    /// This will make it so that every time this type fires it will add it to the end of a file.
    /// You can have multiple files that it will be added to.
    pub fn file(mut self, file: &str) -> Result<Self, Error> {
        match self.working_on {
            Log::Info => self.info_file.push(file.to_string()),
            Log::Warn => self.warn_file.push(file.to_string()),
            Log::Error => self.error_file.push(file.to_string()),
            Log::Fatal => self.fatal_file.push(file.to_string()),
            Log::_None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "You need to set a log type before you can set where it should be sent to",
                ))
            }
        }
        Ok(self)
    }

    /// This will set the currently selected item, this will be used so you can use the other
    /// attributes that require a type to be set.
    pub fn set_type(mut self, log: Log) -> Result<Self, Error> {
        match log {
            Log::_None => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "You can not set the current log type to none",
                ))
            }
            _ => self.working_on = log,
        }
        Ok(self)
    }

    /// This is going to save the configuration.
    pub fn save(self) -> Result<(), PoisonError<MutexGuard<'static, Config>>> {
        let mut config = CONFIG.lock()?;
        *config = self;
        Ok(())
    }
}
