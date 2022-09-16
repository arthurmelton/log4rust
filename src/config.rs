use colored::Color;
use lazy_static::lazy_static;
use std::io::{Error, ErrorKind};
use std::sync::{Mutex, MutexGuard, PoisonError};
use ureq::Request;

lazy_static! {
    #[doc(hidden)]
    pub static ref CONFIG: Mutex<Config> =  Mutex::new(new());
}

#[derive(Clone)]
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

pub enum Console {
    _None,
    Stdout,
    Stderr,
}

/// To configure the logger you will use some different functions. <br>
/// This is how the default configuration would look if written out:
///
/// ```
/// log4rust::new()
///     .time(Time::Local)
///     .set_type(Log::Info)?.color(Color::TrueColor{r:0,g:255,b:255})?.console(Console::Stdout)?.backtrace(Backtrace::_None)?
///     .set_type(Log::Warn)?.color(Color::TrueColor{r:255,g:215,b:185})?.console(Console::Stderr)?.backtrace(Backtrace::_None)?
///     .set_type(Log::Error)?.color(Color::TrueColor{r:255,g:100,b:0})?.console(Console::Stderr)?.backtrace(Backtrace::Simple)?
///     .set_type(Log::Fatal)?.color(Color::TrueColor{r:255,g:0,b:0})?.console(Console::Stderr)?.backtrace(Backtrace::Complex)?
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
    // this is an array with the length of 4 because
    // info = 0
    // warn = 1
    // error = 2
    // fatal = 3
    #[doc(hidden)]
    pub time: Time,
    #[doc(hidden)]
    pub color: [Color; 4],
    #[doc(hidden)]
    pub console: [Console; 4],
    #[doc(hidden)]
    pub web: [Vec<(Request, String)>; 4],
    #[doc(hidden)]
    pub file: [Vec<String>; 4],
    #[doc(hidden)]
    pub backtrace: [Backtrace; 4],
    // private
    working_on: Log,
}

pub fn new() -> Config {
    Config {
        time: Time::Local,
        color: [
            Color::TrueColor {
                r: 0,
                g: 255,
                b: 255,
            },
            Color::TrueColor {
                r: 255,
                g: 215,
                b: 185,
            },
            Color::TrueColor {
                r: 255,
                g: 100,
                b: 0,
            },
            Color::TrueColor { r: 255, g: 0, b: 0 },
        ],
        console: [Console::Stdout, Console::Stderr, Console::Stderr, Console::Stderr],
        web: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        file: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        backtrace: [
            Backtrace::_None,
            Backtrace::_None,
            Backtrace::Simple,
            Backtrace::Complex,
        ],
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
        self.color[index(
            self.working_on.clone(),
            "You need to set a log type before you can set the color",
        )?] = color;
        Ok(self)
    }

    /// This will change the backtrace of a type when it is printed to the console
    pub fn backtrace(mut self, backtrace: Backtrace) -> Result<Self, Error> {
        self.backtrace[index(
            self.working_on.clone(),
            "You need to set a log type before you can set the backtrace",
        )?] = backtrace;
        Ok(self)
    }

    /// This will set if this type will be printed to the console or not
    pub fn console(mut self, console: Console) -> Result<Self, Error> {
        self.console[index(
            self.working_on.clone(),
            "You need to set a log type before you can set if it goes to the terminal",
        )?] = console;
        Ok(self)
    }

    /// This will make it so that you can make a request every time this type goes off. You can
    /// also have multiple of these so it will make multiple requests for every time this type
    /// fires. This would be most useful for a webhook.
    pub fn web(mut self, format: &str, request: Request) -> Result<Self, Error> {
        self.web[index(
            self.working_on.clone(),
            "You need to set a log type before you can set where it should be sent to",
        )?]
        .push((request, format.to_string()));
        Ok(self)
    }

    /// This will make it so that every time this type fires it will add it to the end of a file.
    /// You can have multiple files that it will be added to.
    pub fn file(mut self, file: &str) -> Result<Self, Error> {
        self.file[index(
            self.working_on.clone(),
            "You need to set a log type before you can set where it should be sent to",
        )?]
        .push(file.to_string());
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

fn index(working_on: Log, if_none: &str) -> Result<usize, Error> {
    match working_on {
        Log::Info => Ok(0),
        Log::Warn => Ok(1),
        Log::Error => Ok(2),
        Log::Fatal => Ok(3),
        Log::_None => return Err(Error::new(ErrorKind::Other, if_none)),
    }
}
