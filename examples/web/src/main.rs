use log4rust::*;

fn main() -> Result<(), std::io::Error> {
    log4rust::new()
        .time(Time::Local)
        .set_type(Log::Info)?.web(
            "{\"Type\":\"Info\",\"Body\":\"{}\"}",
            ureq::post("https://Loggerz.free.beeceptor.com")
                .set("Content-Type", "application/json"),
        )?
        .set_type(Log::Warn)?.web(
            "{\"Type\":\"Warn\",\"Body\":\"{}\"}",
            ureq::post("https://Loggerz.free.beeceptor.com")
                .set("Content-Type", "application/json"),
        )?
        .set_type(Log::Error)?.web(
            "{\"Type\":\"Error\",\"Body\":\"{}\"}",
            ureq::post("https://Loggerz.free.beeceptor.com")
                .set("Content-Type", "application/json"),
        )?
        .set_type(Log::Fatal)?.web(
            "{\"Type\":\"Fatal\",\"Body\":\"{}\"}",
            ureq::post("https://Loggerz.free.beeceptor.com")
                .set("Content-Type", "application/json"),
        )?
        .save()
        .unwrap();
    // go to https://beeceptor.com/console/loggerz to see this happen
    info!("This is some info");
    warn!("This is a warning");
    error!("This is an error");
    fatal!("This is something fatal");
    Ok(())
}
