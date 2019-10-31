//! This crate provides `macro_rules!` data structures for storing and manipulating tokens.
//!
//! The macros provided by this crate are compatible with [`tt_call`], making them suitable
//! for use in modular tt-muncher macros.  It is recommended that you read the [`tt_call`]
//! documentation before attempting to use this crate.
//!
//! # List token structure
//!
//! The smallest token list is the single identifier `tt_list`, which represents an empty list.
//! Items in the list are surrounded by  `[{ }]`.
//!
//! The following list contains 3 items (`foo`, `bar 1.0`, and `{ baz }`):
//! ```
//! tt_list [{ foo }] [{ bar 1.0 }] [{ { baz } }]
//! ```
//!
//! [`tt_call`]: ../tt_call/index.html

#[macro_use]
mod list;
