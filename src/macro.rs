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

                    let index = match $type {
                        "info" => 0,
                        "warn" => 1,
                        "error" => 2,
                        _ => 3
                    };

                    let backtrace_string = match &config.backtrace[index] {
                        Backtrace::_None => "".to_string(),
                        Backtrace::Simple => format!(" ({}:{}:{})", file!(), line!(), column!()),
                        Backtrace::Complex => format!("\n{:?}", log4rust::backtrace::Backtrace::new()),
                    };

                    let text = format!("{} {}{}", time, $write, backtrace_string);

                    if config.console[index] {
                        println!("{}", text.color(config.color[index]));
                    }

                    for (request, format) in &config.web[index] {
                        match request.clone().send_string(&format.replace("{}", &text)) {
                            Ok(_) => {},
                            Err(e) => println!("{}", format!("Failed to send a http request as a {} and wanted to send '{}', Request error: {:?}", $type, text, e).color(config.color[3])),
                        }
                    }

                    for i in &config.file[index] {
                        match OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open(i) {
                                Ok(mut file) => match writeln!(file, "{}", text) {
                                    Ok(_) => {},
                                    Err(e) => println!("{}", format!("Couldn't write to file: {}, wanted to write '{}' as a {}, error: {:?}", e, text, $type, e).color(config.color[3])),
                                },
                                Err(e) => println!("{}", format!("Couldn't create OpenOptions for file {}, wanted to write {} as a {}, error: {:?}", i, text, $type, e).color(config.color[3])),
                            }
                    }
                },
                Err(e) => println!("FATAL!!!: Faild to get the config, wanted to make a {} with the text '{}', error: {:?}", $type, format!("[{}] {}", Utc::now(), $write), e),
            }
        }
    };
}
