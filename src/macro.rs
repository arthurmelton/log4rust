#[macro_export]
macro_rules! info {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!("info", format!($($x,)*));
        }
    };
}

#[macro_export]
macro_rules! warn {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!("warn", format!($($x,)*));
        }
    };
}

#[macro_export]
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!("error", format!($($x,)*));
        }
    };
}

#[macro_export]
macro_rules! fatal {
    ( $( $x:expr ),* ) => {
        {
            log4rust_template!("fatal", format!($($x,)*));
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

                    let backtrace = match $type {
                        "info" => &config.info_backtrace,
                        "warn" => &config.warn_backtrace,
                        "error" => &config.error_backtrace,
                        _ => &config.fatal_backtrace,
                    };

                    let backtrace_string = match backtrace {
                        Backtrace::_None => "".to_string(),
                        Backtrace::Simple => format!(" ({}:{}:{})", file!(), line!(), column!()),
                        Backtrace::Complex => format!("\n{:?}", log4rust::backtrace::Backtrace::new()),
                    };

                    let text = format!("{} {}{}", time, $write, backtrace_string);
                    
                    let console = match $type {
                        "info" => config.info_console,
                        "warn" => config.warn_console,
                        "error" => config.error_console,
                        _ => config.fatal_console
                    };

                    if console {
                        println!("{}", text.color(match $type {
                            "info" => config.info_color,
                            "warn" => config.warn_color,
                            "error" => config.error_color,
                            _ => config.fatal_color,
                        }));
                    }

                    let web = match $type {
                        "info" => &config.info_web,
                        "warn" => &config.warn_web,
                        "error" => &config.error_web,
                        _ => &config.fatal_web
                    };

                    for (request, format) in web {
                        match request.clone().send_string(&format.replace("{}", &text)) {
                            Ok(_) => {},
                            Err(e) => println!("{}", format!("Failed to send a http request as a {} and wanted to send '{}', Request error: {:?}", $type, text, e).color(config.fatal_color)),
                        }
                    }

                    let file = match $type {
                        "info" => &config.info_file,
                        "warn" => &config.warn_file,
                        "error" => &config.error_file,
                        _ => &config.fatal_file
                    };

                    for i in file {
                        match OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(i) {
                                Ok(mut file) => match writeln!(file, "{}", text) {
                                    Ok(_) => {},
                                    Err(e) => println!("{}", format!("Couldn't write to file: {}, wanted to write '{}' as a {}, error: {:?}", e, text, $type, e).color(config.fatal_color)),
                                },
                                Err(e) => println!("{}", format!("Couldn't create OpenOptions for file {}, wanted to write {} as a {}, error: {:?}", i, text, $type, e).color(config.fatal_color)),
                            }
                    }
                },
                Err(e) => println!("FATAL!!!: Faild to get the config, wanted to make a {} with the text '{}', error: {:?}", $type, format!("[{}] {}", Utc::now(), $write), e),
            }
        }
    };
}
