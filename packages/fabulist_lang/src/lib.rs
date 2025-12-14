#![warn(missing_docs)]

//! # Fabulist Language Library
//!
//! Core components for the Fabulist language: abstract syntax tree (AST) building blocks,
//! a lightweight runtime environment, intrinsic helpers, and a pest-based parser. Use the
//! [`FabulistParser`] to turn a story source into an AST; lower-level nodes implement
//! [`Evaluable`] so expressions and statements can be interpreted when needed.
//!
//! ```rust
//! use fabulist_lang::parser::FabulistParser;
//! let source_code = r##"
//! story { "start": "dialogue_1" }
//!
//! ## dialogue_1
//! [Jose]
//! > "What's up"
//!     - "The ceiling." => {
//!         "next": () => {
//!             goto dialogue_2;
//!         },
//!         "change_context": () => {
//!             context.mood = "annoyed";
//!         }
//!     }
//!     - "Nothing much." => {
//!         "next": () => {
//!            goto dialogue_2;
//!         }
//!     }
//! "##;
//!
//! println!("Source Code:\n{}", source_code);
//!
//! let ast = FabulistParser::parse(source_code).expect("parse failure");
//! assert_eq!(ast.parts.len(), 1);
//! ```
//!
//! ## Licensing
//!
//! Licensed under either (at your option):
//!   * MIT license; or
//!   * Apache License, Version 2.0
//!
//! Copyright (c) 2025 Daveren John Reyes Cordero
//!
//! [`Evaluable`]: crate::interpreter::Evaluable
//! [`FabulistParser`]: crate::parser::FabulistParser
//! [`fabulist_core`]: https://crates.io/crates/fabulist_core

pub mod error;
pub mod interpreter;
pub mod parser;
