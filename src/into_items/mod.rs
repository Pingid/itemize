use crate::iter::ItemsFromIter;

mod tuples;

/// Consume or borrow an input and yield an iterator of `T`.
///
/// Implementations exist for tuples, collections, and references so APIs can accept
/// whichever shape is convenient for the caller while you always receive `T`.
pub trait IntoItems<T> {
    type IntoIter: Iterator<Item = T>;
    fn into_items(self) -> Self::IntoIter;
}

impl<T, C> IntoItems<T> for C
where
    C: ItemsFromIter,
    T: From<C::Item>,
{
    type IntoIter = std::iter::Map<C::IntoIter, fn(C::Item) -> T>;

    fn into_items(self) -> Self::IntoIter {
        fn map_conv<T, A>(a: A) -> T
        where
            T: From<A>,
        {
            T::from(a)
        }

        ItemsFromIter::items_from_iter(self).map(map_conv::<T, C::Item>)
    }
}

/// Fallible counterpart to [`IntoItems`] that surfaces conversion errors directly.
pub trait TryIntoItems<T, E> {
    type IntoIter: Iterator<Item = Result<T, E>>;
    fn try_into_items(self) -> Self::IntoIter;
}

impl<T, E, C> TryIntoItems<T, E> for C
where
    C: ItemsFromIter,
    T: TryFrom<C::Item, Error = E>,
{
    type IntoIter = std::iter::Map<C::IntoIter, fn(C::Item) -> Result<T, E>>;
    fn try_into_items(self) -> Self::IntoIter {
        ItemsFromIter::items_from_iter(self).map(|item| T::try_from(item))
    }
}
