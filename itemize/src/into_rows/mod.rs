use crate::iter::ItemsFromIter;
use crate::{IntoItems, TryIntoItems};

mod tuples;
mod variadic;

#[cfg(feature = "into_rows_variadic")]
pub use variadic::*;

/// Convert 2D inputs into iterators of rows whose elements end up as `T`.
///
/// Works with tuples of tuples, slices of arrays, references, or anything that
/// implements [`IntoItems`]. Ownership is preserved automatically so you can accept
/// both owned and borrowed data with the same signature.
pub trait IntoRows<T> {
    type RowIter: Iterator<Item = T>;
    type Rows: Iterator<Item = Self::RowIter>;

    fn into_rows(self) -> Self::Rows;
}

impl<T, C> IntoRows<T> for C
where
    C: ItemsFromIter,
    <C as ItemsFromIter>::Item: IntoItems<T>,
{
    type RowIter = <<C as ItemsFromIter>::Item as IntoItems<T>>::IntoIter;

    type Rows = std::iter::Map<
        <C as ItemsFromIter>::IntoIter,
        fn(<C as ItemsFromIter>::Item) -> Self::RowIter,
    >;

    fn into_rows(self) -> Self::Rows {
        <C as ItemsFromIter>::items_from_iter(self).map(|item| item.into_items())
    }
}

/// Fallible counterpart to [`IntoRows`] for cases where `T` is produced via `TryFrom`.
pub trait TryIntoRows<T, E> {
    type RowIter: Iterator<Item = Result<T, E>>;
    type Rows: Iterator<Item = Self::RowIter>;

    fn try_into_rows(self) -> Self::Rows;
}

impl<T, E, C> TryIntoRows<T, E> for C
where
    C: ItemsFromIter,
    <C as ItemsFromIter>::Item: TryIntoItems<T, E>,
{
    type RowIter = <<C as ItemsFromIter>::Item as TryIntoItems<T, E>>::IntoIter;

    type Rows = std::iter::Map<
        <C as ItemsFromIter>::IntoIter,
        fn(<C as ItemsFromIter>::Item) -> Self::RowIter,
    >;

    fn try_into_rows(self) -> Self::Rows {
        <C as ItemsFromIter>::items_from_iter(self).map(|item| item.try_into_items())
    }
}
