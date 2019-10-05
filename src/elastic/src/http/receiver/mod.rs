/*!
Low-level http client details for response parsing.

This module contains low-level implementation details for the high-level [`Client`][Client].

[Client]: ../struct.Client.html
*/

#[cfg(feature="async_sender")]
mod asynchronous;
mod error;
mod parsing;
#[cfg(feature="sync_sender")]
mod synchronous;

pub use self::{
    error::*,
    parsing::*,
};

#[cfg(feature="async_sender")]
pub use self::asynchronous::*;
#[cfg(feature="sync_sender")]
pub use self::synchronous::*;
