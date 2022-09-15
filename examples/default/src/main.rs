use log4rust::*;

fn main() -> Result<(), std::io::Error> {
    log4rust::new()
        .time(Time::Local)
        .set_type(Log::Info)?.color(Color::TrueColor{r:0,g:255,b:255})?.console(true)?.backtrace(Backtrace::_None)?
        .set_type(Log::Warn)?.color(Color::TrueColor{r:255,g:215,b:185})?.console(true)?.backtrace(Backtrace::_None)?
        .set_type(Log::Error)?.color(Color::TrueColor{r:255,g:100,b:0})?.console(true)?.backtrace(Backtrace::Simple)?
        .set_type(Log::Fatal)?.color(Color::TrueColor{r:255,g:0,b:0})?.console(true)?.backtrace(Backtrace::Complex)?
        .save()
        .unwrap();
    info!("This is some info");
    warn!("This is a warning");
    error!("This is an error");
    fatal!("This is something fatal");
    Ok(())
}
