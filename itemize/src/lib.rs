//! Write APIs that accept single values, tuples, or collections—then work with a predictable iterator inside your function.
//!
//! # Core Traits
//!
//! This crate provides four traits for flattening heterogeneous inputs into iterators:
//!
//! - [`IntoItems<T>`] – flatten inputs into `Iterator<Item = T>`
//! - [`TryIntoItems<T, E>`] – flatten inputs into `Iterator<Item = Result<T, E>>`
//! - [`IntoRows<T>`] – flatten 2D inputs into nested iterators
//! - [`TryIntoRows<T, E>`] – flatten 2D inputs with fallible conversion
//!
//! These traits let you write functions that accept a single value, a tuple of values,
//! or a collection, and handle them uniformly through iteration.
//!
//! # Derive Macros
//!
//! Generate trait implementations with the `#[items_from(...)]` attribute:
//!
//! - `types(...)` – types to accept (e.g., `String`, `&'a str`, `usize`)
//! - `tuples(n)` – support tuples up to size n
//! - `collections(vec, slice, array)` – which collection types to support
//! - `error_type(Type)` – lock `TryInto*` impls to a specific error type
//!
//! # Examples
//!
//! ## Basic `IntoItems`
//!
//! ```rust
//! use itemize::IntoItems;
//!
//! #[derive(Debug, IntoItems)]
//! #[items_from(types(u32), tuples(2), collections(vec, array))]
//! struct Count(u32);
//!
//! impl From<u32> for Count {
//!     fn from(value: u32) -> Self {
//!         Count(value)
//!     }
//! }
//!
//! fn collect_counts(input: impl IntoItems<Count>) -> Vec<u32> {
//!     input.into_items().map(|Count(n)| n).collect()
//! }
//!
//! // Accepts single values, tuples, and collections
//! assert_eq!(collect_counts(5), vec![5]);
//! assert_eq!(collect_counts((1, 2)), vec![1, 2]);
//! assert_eq!(collect_counts([3, 4, 5]), vec![3, 4, 5]);
//! assert_eq!(collect_counts(vec![6, 7]), vec![6, 7]);
//! ```
//!
//! ## Fallible Parsing with `TryIntoItems`
//!
//! ```rust
//! use itemize::TryIntoItems;
//!
//! #[derive(Debug)]
//! struct ParseError(String);
//!
//! impl From<std::num::ParseIntError> for ParseError {
//!     fn from(e: std::num::ParseIntError) -> Self {
//!         ParseError(e.to_string())
//!     }
//! }
//!
//! #[derive(TryIntoItems)]
//! #[items_from(types(String, &'a str, i64), tuples(3), collections(vec, slice, array), error_type(ParseError))]
//! struct Int(i64);
//!
//! impl TryFrom<&str> for Int {
//!     type Error = std::num::ParseIntError;
//!     fn try_from(s: &str) -> Result<Self, Self::Error> {
//!         s.parse::<i64>().map(Int)
//!     }
//! }
//!
//! impl TryFrom<String> for Int {
//!     type Error = std::num::ParseIntError;
//!     fn try_from(s: String) -> Result<Self, Self::Error> {
//!         s.parse::<i64>().map(Int)
//!     }
//! }
//!
//! impl TryFrom<i64> for Int {
//!     type Error = std::num::ParseIntError;
//!     fn try_from(n: i64) -> Result<Self, Self::Error> {
//!         Ok(Int(n))
//!     }
//! }
//!
//! fn parse_ints(input: impl TryIntoItems<Int, ParseError>) -> Result<Vec<Int>, ParseError> {
//!     input.try_into_items().collect()
//! }
//!
//! # fn main() -> Result<(), ParseError> {
//! // Accepts various input formats
//! let _ = parse_ints("42")?;
//! let _ = parse_ints(("1", "2".to_string(), 3))?;
//! let _ = parse_ints(["10", "20", "30"])?;
//! # Ok(())
//! # }
//! ```
//!
//! ## 2D Flattening with `IntoRows`
//!
//! ```rust
//! use itemize::{IntoItems, IntoRows};
//!
//! #[derive(Debug, IntoItems, IntoRows)]
//! #[items_from(types(&'a str), tuples(3))]
//! struct Cell<'a>(&'a str);
//!
//! impl<'a> From<&'a str> for Cell<'a> {
//!     fn from(value: &'a str) -> Self {
//!         Cell(value)
//!     }
//! }
//!
//! fn collect_rows<'a>(input: impl IntoRows<Cell<'a>>) -> Vec<Vec<&'a str>> {
//!     input
//!         .into_rows()
//!         .map(|row| row.map(|Cell(value)| value).collect())
//!         .collect()
//! }
//!
//! // Each tuple in the outer tuple becomes a row
//! let grid = collect_rows((("a", "b"), ("c", "d", "e")));
//! assert_eq!(grid, vec![vec!["a", "b"], vec!["c", "d", "e"]]);
//! ```
//!
//! # Trait Bounds
//!
//! - `IntoItems<T>` implementations expect `T: From<Source>` for every declared source type.
//! - `TryIntoItems<T, E>` implementations expect `T: TryFrom<Source, Error = SourceErr>` with `SourceErr: Into<E>`.
//!
//! # Additional Types
//!
//! The [`Either`] type enables composing heterogeneous iterators through nesting.
//! It allows two different iterator types to be unified into a single type.

#[cfg(feature = "derive")]
pub use itemize_derive::*;

pub mod either;
pub use either::Either;

pub trait IntoItems<Item> {
    type IntoIter: ::std::iter::Iterator<Item = Item>;
    fn into_items(self) -> Self::IntoIter;
}

pub trait TryIntoItems<Item, E> {
    type IntoIter: ::std::iter::Iterator<Item = Result<Item, E>>;
    fn try_into_items(self) -> Self::IntoIter;
}

pub trait IntoRows<Row> {
    type RowIter: ::std::iter::Iterator<Item = Row>;
    type Rows: ::std::iter::Iterator<Item = Self::RowIter>;
    fn into_rows(self) -> Self::Rows;
}

pub trait TryIntoRows<Row, E> {
    type RowIter: ::std::iter::Iterator<Item = Result<Row, E>>;
    type Rows: ::std::iter::Iterator<Item = Self::RowIter>;
    fn try_into_rows(self) -> Self::Rows;
}
