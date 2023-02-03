#[macro_export]
macro_rules! info {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!(log4rust::Log::Info, format!($($x,)*));
        }
    };
}

#[macro_export]
macro_rules! warn {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!(log4rust::Log::Warn, format!($($x,)*));
        }
    };
}

#[macro_export]
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!(log4rust::Log::Error, format!($($x,)*));
        }
    };
}

#[macro_export]
macro_rules! fatal {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!(log4rust::Log::Fatal, format!($($x,)*));
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! log4rust_template {
    ( $type:expr, $write:expr ) => {
        {
            use log4rust::*;
            use log4rust::chrono::prelude::*;
            use std::fs::OpenOptions;
            use std::io::Write;
            match log4rust::CONFIG.lock() {
                Ok(config) => {
                    let time = if config.time == Time::UTC {format!("[{}]", Utc::now())}
                    else {format!("[{}]", Local::now())};

                    let index = match $type {
                        Log::Info => 0,
                        Log::Warn => 1,
                        Log::Error => 2,
                        _ => 3
                    };

                    let backtrace_string = match &config.backtrace[index] {
                        Backtrace::_None => "".to_string(),
                        Backtrace::Simple => format!(" ({}:{}:{})", file!(), line!(), column!()),
                        Backtrace::Complex => format!("\n{:?}", log4rust::backtrace::Backtrace::new()),
                    };

                    let text = format!("{} {}{}", time, $write, backtrace_string);

                    match config.console[index] {
                        Console::_None => {}
                        Console::Stdout => println!("{}", text.color(config.color[index])),
                        Console::Stderr => eprintln!("{}", text.color(config.color[index])),
                    }

                    web(&config, index, text.clone(), $type);

                    for i in &config.file[index] {
                        match OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(i) {
                                Ok(mut file) => match writeln!(file, "{}", text) {
                                    Ok(_) => {},
                                    Err(e) => println!("{}", format!("Couldn't write to file: {}, wanted to write '{}' as a {:?}, error: {:?}", e, text, $type, e).color(config.color[3])),
                                },
                                Err(e) => println!("{}", format!("Couldn't create OpenOptions for file {}, wanted to write {} as a {:?}, error: {:?}", i, text, $type, e).color(config.color[3])),
                            }
                    }
                },
                Err(e) => println!("FATAL!!!: Faild to get the config, wanted to make a {:?} with the text '{}', error: {:?}", $type, format!("[{}] {}", Utc::now(), $write), e),
            }
        }
    };
}

#[cfg(feature = "web")]
use colored::Colorize;
use std::sync::MutexGuard;

#[doc(hidden)]
pub fn web(_config: &MutexGuard<'_, crate::Config>, _index: usize, _text:String, _item: crate::Log) {
    #[cfg(feature = "web")]
    for (request, format) in &_config.web[_index] {
        match request.clone().send_string(&format.replace("{}", &_text)) {
            Ok(_) => {},
            Err(e) => println!("{}", format!("Failed to send a http request as a {:?} and wanted to send '{}', Request error: {:?}", _item, _text, e).color(_config.color[3])),
        }
    }
} 
