//! A library used to interact with a VLC player's TCP interface.
//!
//! Primary type:
//!
//! * [`Client`] - Represents a connection to VLC's TCP interface.

mod error;

pub mod client;

pub use client::Client;
pub use error::Error;

/// A crate-level result that may be returned when working with VLC.
pub type Result<T> = std::result::Result<T, Error>;
