//! Pure deterministic configuration contracts. No gameplay engine is implemented.
#![forbid(unsafe_code)]

mod board;
mod config;
mod lobby;
pub use board::*;
pub use config::*;
pub use lobby::*;
