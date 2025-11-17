use crate::iter::ItemsFromIter;
use crate::{IntoItems, TryIntoItems};

mod tuples;
mod variadic;

#[cfg(feature = "into_rows_variadic")]
pub use variadic::*;

/// A trait for converting 2D collections into nested iterators with element conversion.
///
/// This trait enables uniform iteration over nested collection types, where each "row"
/// is converted to an iterator of items. The trait handles both owned and borrowed
/// collections, preserving ownership semantics throughout the iteration chain.
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
