//! ECMAScript compliant pure Rust implementation of Ryū, an algorithm to quickly
//! convert floating point numbers to decimal strings.
//!
//! The PLDI'18 paper [*Ryū: fast float-to-string conversion*][paper] by Ulf
//! Adams includes a complete correctness proof of the algorithm. The paper is
//! available under the creative commons CC-BY-SA license.
//!
//! This Rust implementation is a line-by-line port of Ulf Adams' implementation
//! in C, [https://github.com/ulfjack/ryu][upstream].
//!
//! [paper]: https://dl.acm.org/citation.cfm?id=3192369
//! [upstream]: https://github.com/ulfjack/ryu
//!
//! # Example
//!
//! ```
//! fn main() {
//!     let mut buffer = ryu_js::Buffer::new();
//!     let printed = buffer.format(1.234);
//!     assert_eq!(printed, "1.234");
//! }
//! ```
//!
//! ## Performance
//!
//! The benchmarks measure the average time to print a 32-bit float and average
//! time to print a 64-bit float, where the inputs are distributed as uniform random
//! bit patterns 32 and 64 bits wide.
//!
//! The upstream C code, the unsafe direct Rust port, and the safe pretty Rust API
//! all perform the same, taking around 21 nanoseconds to format a 32-bit float and
//! 31 nanoseconds to format a 64-bit float.
//!
//! There is also a Rust-specific benchmark comparing this implementation to the
//! standard library which you can run with:
//!
//! ```console
//! $ cargo bench
//! ```
//!
//! The benchmark shows Ryū approximately 4-10x faster than the standard library
//! across a range of f32 and f64 inputs. Measurements are in nanoseconds per
//! iteration; smaller is better.
//!
//! | type=f32 | 0.0  | 0.1234 | 2.718281828459045 | f32::MAX |
//! |:--------:|:----:|:------:|:-----------------:|:--------:|
//! | RYU      | 3ns  | 28ns   | 23ns              | 22ns     |
//! | STD      | 40ns | 106ns  | 128ns             | 110ns    |
//!
//! | type=f64 | 0.0  | 0.1234 | 2.718281828459045 | f64::MAX |
//! |:--------:|:----:|:------:|:-----------------:|:--------:|
//! | RYU      | 3ns  | 50ns   | 35ns              | 32ns     |
//! | STD      | 39ns | 105ns  | 128ns             | 202ns    |
//!
//! ## Formatting
//!
//! This library tends to produce more human-readable output than the standard
//! library's to\_string, which never uses scientific notation. Here are two
//! examples:
//!
//! - *ryu:* 1.23e40, *std:* 12300000000000000000000000000000000000000
//! - *ryu:* 1.23e-40, *std:* 0.000000000000000000000000000000000000000123
//!
//! Both libraries print short decimals such as 0.0000123 without scientific
//! notation.

#![no_std]
#![doc(html_root_url = "https://docs.rs/ryu-js/0.2.1")]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::checked_conversions,
    clippy::doc_markdown,
    clippy::expl_impl_clone_on_copy,
    clippy::if_not_else,
    clippy::many_single_char_names,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix,
    clippy::wildcard_imports
)]

mod buffer;
mod common;
mod d2s;
#[cfg(not(feature = "small"))]
mod d2s_full_table;
mod d2s_intrinsics;
#[cfg(feature = "small")]
mod d2s_small_table;
mod digit_table;
mod f2s;
mod f2s_intrinsics;
mod pretty;

pub use crate::buffer::{Buffer, Float};

/// Unsafe functions that mirror the API of the C implementation of Ryū.
pub mod raw {
    pub use crate::pretty::{format32, format64};
}
