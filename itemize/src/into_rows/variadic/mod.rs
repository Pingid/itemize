use crate::iter::ItemsFromIter;
use crate::{IntoItems, TryIntoItems};

mod either;
mod tuples;
pub trait IntoVariadicRows<T> {
    type RowIter: Iterator<Item = T>;
    type Rows: Iterator<Item = Self::RowIter>;

    fn into_variadic_rows(self) -> Self::Rows;
}

impl<T, C> IntoVariadicRows<T> for C
where
    C: ItemsFromIter,
    <C as ItemsFromIter>::Item: IntoItems<T>,
{
    type RowIter = <<C as ItemsFromIter>::Item as IntoItems<T>>::IntoIter;

    type Rows = std::iter::Map<
        <C as ItemsFromIter>::IntoIter,
        fn(<C as ItemsFromIter>::Item) -> Self::RowIter,
    >;

    fn into_variadic_rows(self) -> Self::Rows {
        fn map_row<T, R>(row: R) -> <R as IntoItems<T>>::IntoIter
        where
            R: IntoItems<T>,
        {
            row.into_items()
        }

        <C as ItemsFromIter>::items_from_iter(self).map(map_row::<T, <C as ItemsFromIter>::Item>)
    }
}

/// Fallible counterpart to [`IntoVariadicRows`].
pub trait TryIntoVariadicRows<T, E> {
    type RowIter: Iterator<Item = Result<T, E>>;
    type Rows: Iterator<Item = Self::RowIter>;

    fn try_into_variadic_rows(self) -> Self::Rows;
}

impl<T, E, C> TryIntoVariadicRows<T, E> for C
where
    C: ItemsFromIter,
    <C as ItemsFromIter>::Item: TryIntoItems<T, E>,
{
    type RowIter = <<C as ItemsFromIter>::Item as TryIntoItems<T, E>>::IntoIter;
    type Rows = std::iter::Map<
        <C as ItemsFromIter>::IntoIter,
        fn(<C as ItemsFromIter>::Item) -> Self::RowIter,
    >;

    fn try_into_variadic_rows(self) -> Self::Rows {
        <C as ItemsFromIter>::items_from_iter(self).map(|item| item.try_into_items())
    }
}
