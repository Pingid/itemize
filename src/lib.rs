//! Itemize lets your APIs accept tuples, slices, collections, or single values while
//! you always work with a predictable iterator of the target type.
//!
//! ## Core traits
//!
//! - [`IntoItems`] and [`TryIntoItems`] consume or borrow 1D inputs and yield iterators
//!   of a caller-defined type via [`From`] / [`TryFrom`]. Tuples, `Vec`, `Option`,
//!   `Result`, references, and most standard collections already implement the trait.
//! - [`IntoRows`] and [`TryIntoRows`] (feature `into_rows`) lift nested inputs such as
//!   tuples of tuples, slices of arrays, or borrowed collections into iterators of rows
//!   where each row produces `T`.
//! - [`IntoVariadicRows`] and [`TryIntoVariadicRows`] (feature `into_rows_variadic`)
//!   handle heterogeneous row shapes by wrapping iterators with an `Either`
//!   enum so the caller still receives a single iterator type.
//!
//! ## Internal plumbing
//!
//! The internal `ItemsFromIter` trait mirrors `IntoIterator` and powers the blanket
//! implementations for collections without conflicting with tuple impls. It is kept
//! private to avoid exposing extra traits to downstream crates.
//!
//! ## Example
//!
//! ```
//! use itemize::{TryIntoItems, TryIntoVariadicRows};
//!
//! enum ParseError {
//!     BadInt(String),
//! }
//!
//! struct Answer(i64);
//!
//! impl TryFrom<&str> for Answer {
//!     type Error = ParseError;
//!     fn try_from(s: &str) -> Result<Self, Self::Error> {
//!         s.parse::<i64>()
//!             .map(Answer)
//!             .map_err(|_| ParseError::BadInt(s.to_string()))
//!     }
//! }
//!
//! impl TryFrom<f64> for Answer {
//!     type Error = ParseError;
//!     fn try_from(n: f64) -> Result<Self, Self::Error> {
//!         Ok(Answer(n as i64))
//!     }
//! }
//!
//! impl TryFrom<usize> for Answer {
//!     type Error = ParseError;
//!     fn try_from(n: usize) -> Result<Self, Self::Error> {
//!         Ok(Answer(n as i64))
//!     }
//! }
//!
//! fn parse_ints(inputs: impl TryIntoItems<Answer, ParseError>) -> Result<Vec<Answer>, ParseError> {
//!     inputs.try_into_items().collect()
//! }
//!
//! fn parse_int_rows(
//!     inputs: impl TryIntoVariadicRows<Answer, ParseError>,
//! ) -> Vec<Result<Vec<Answer>, ParseError>> {
//!     inputs
//!         .try_into_variadic_rows()
//!         .map(|rows| rows.collect())
//!         .collect()
//! }
//!
//! fn demo() -> Result<(), ParseError> {
//!     // works with [&str]
//!     let values = parse_ints(["1", "2", "3"]);
//!     // works with tuples
//!     let more = parse_ints(("1", 2, 3.0))?;
//!     // works with heterogeneous tuples
//!     let rows = parse_int_rows((("1", 2, 3.0), ("4", "5")));
//!     Ok(())
//! }
//! ```

pub(crate) mod iter;

mod into_items;
pub use into_items::*;

#[cfg(feature = "into_rows")]
mod into_rows;
#[cfg(feature = "into_rows")]
pub use into_rows::*;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Custom(String);

    impl<T> From<T> for Custom
    where
        T: ToString,
    {
        fn from(value: T) -> Self {
            Custom(value.to_string())
        }
    }

    fn into_items(x: impl IntoItems<Custom>) -> Vec<Custom> {
        x.into_items().collect()
    }

    #[test]
    fn test_into_items_for_collections() {
        // single values
        into_items(("1",));
        into_items((3.14f64,));
        into_items(("1".to_string(),));

        // collections
        into_items([1, 2, 3]);
        into_items(&[1, 2, 3]);
        into_items(vec![1, 2, 3]);
        into_items(&vec![1, 2, 3]);
        into_items(HashSet::from([1, 2, 3]));
        into_items(&HashSet::from([1, 2, 3]));

        // tuples
        into_items((1, "2", 3f64));
        into_items(&(1, "2", 3f64));
        into_items(&(1, &"2", 3f64));
    }

    fn into_rows(x: impl IntoRows<Custom>) -> Vec<Vec<Custom>> {
        x.into_rows().map(|row| row.collect()).collect()
    }

    #[test]
    fn test_into_rows_for_collections() {
        // single values
        into_rows(((1,),));
        into_rows(((3.14f64,),));
        into_rows((("1".to_string(),),));

        // collections
        into_rows(vec![&vec![1, 2, 3], &vec![4, 5, 6]]);
        into_rows(&vec![[1, 2, 3], [4, 5, 6]]);

        // tuples
        into_rows(((1, "2", 3f64), (4, "5", 6f64)));
        into_rows((&(1, "2", 3f64), &(4, "5", 6f64)));
        into_rows((&(1, &"2", 3f64), &(4, &"5", 6f64)));
    }

    fn into_variadic_rows(x: impl IntoVariadicRows<Custom>) -> Vec<Vec<Custom>> {
        x.into_variadic_rows().map(|row| row.collect()).collect()
    }

    #[test]
    fn test_into_variadic_rows_for_tuples() {
        assert_eq!(
            into_variadic_rows(((1, "2"), (4, "5", 6f64))),
            vec![into_items(("1", "2")), into_items(("4", "5", "6")),]
        );
    }
}
