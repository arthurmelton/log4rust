# Log4rust

A logging system for rust that is trying to be as easy as possibale to impliment

## Example

```rs
fn main() {
    log4rust::new().save().unwrap();
    // this could be in a new thread or anywhere in your code
    info!("This is some info");
    warn!("This is a warning");
    error!("This is an error");
    fatal!("This is something fatal");
}
```

You can also look in the exmaples folder to see how to do some other things

Crate: [https://crates.io/crates/log4rust](https://crates.io/crates/log4rust) <br>
Docs: [https://docs.rs/log4rust](https://docs.rs/log4rust)
