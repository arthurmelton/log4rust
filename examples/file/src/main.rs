use log4rust::*;

fn main() -> Result<(), std::io::Error> {
    log4rust::new()
        .time(Time::Local)
        .set_type(Log::Info)?.file("info.txt")?.file("all.txt")?
        .set_type(Log::Warn)?.file("warn.txt")?.file("all.txt")?
        .set_type(Log::Error)?.file("error.txt")?.file("all.txt")?
        .set_type(Log::Fatal)?.file("fatal.txt")?.file("all.txt")?
        .save()
        .unwrap();
    info!("This is some info");
    warn!("This is a warning");
    error!("This is an error");
    fatal!("This is something fatal");
    Ok(())
}
