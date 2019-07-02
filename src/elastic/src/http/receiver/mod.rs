/*!
Low-level http client details for response parsing.

This module contains low-level implementation details for the high-level [`Client`][Client].

[Client]: ../struct.Client.html
*/

mod asynchronous;
mod error;
mod parsing;
mod synchronous;

pub use self::{
    asynchronous::*,
    error::*,
    parsing::*,
    synchronous::*,
};
