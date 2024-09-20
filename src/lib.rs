#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use]
pub mod parser;
pub use parser::TiDBParser;

pub mod statement;

pub mod admin;
