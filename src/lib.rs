#![crate_type = "lib"]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Loggerz is a logging system that is made to be as easy as possiable! <br>
//! Look in [config](https://docs.rs/log4rust/latest/log4rust/struct.Config.html) to see how to customize the logger.
//!
//! # Example
//! ```
//! fn main() {
//!     log4rust::new().save().unwrap();
//!     // this could be in a new thread or anywhere in your code
//!     info!("This is some info");
//!     warn!("This is a warning");
//!     error!("This is an error");
//!     fatal!("This is something fatal");
//! }
//! ```

mod config;
mod r#macro;
#[doc(hidden)]
pub use backtrace;
#[doc(hidden)]
pub use chrono;
pub use colored::Color;
#[doc(hidden)]
pub use colored::Colorize;
pub use config::*;
#[doc(hidden)]
pub use r#macro::*;
#[cfg(feature = "web")]
pub use ureq;
