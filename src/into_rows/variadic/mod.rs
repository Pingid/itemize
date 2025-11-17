use crate::iter::ItemsFromIter;
use crate::{IntoItems, TryIntoItems};

mod either;
mod tuples;

pub use either::Either;

/// A trait for converting heterogeneous 2D collections into nested iterators.
///
/// This trait enables iteration over collections where rows may have different concrete
/// types but all yield the same item type `T`. It uses the `Either` enum to unify
/// different iterator types into a single type, allowing heterogeneous collections
/// like tuples of different collection types to be treated uniformly.
///
/// # Examples
///
/// ```
/// use itemize::IntoVariadicRows;
///
/// // Mix different collection types in a tuple
/// let data = (vec![1, 2], [3, 4]);
/// let mut rows = data.into_variadic_rows();
///
/// let first_row: Vec<i32> = rows.next().unwrap().collect();
/// assert_eq!(first_row, vec![1, 2]);
///
/// let second_row: Vec<i32> = rows.next().unwrap().collect();
/// assert_eq!(second_row, vec![3, 4]);
///
/// // Works with different sized collections
/// let mixed = (vec![1, 2, 3], [4, 5]);
/// let items: Vec<Vec<i32>> = mixed.into_variadic_rows()
///     .map(|row| row.collect::<Vec<i32>>())
///     .collect();
/// assert_eq!(items, vec![vec![1, 2, 3], vec![4, 5]]);
/// ```
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
