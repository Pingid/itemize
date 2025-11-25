/// Either type for composing heterogeneous iterators through nesting.
///
/// This enum allows two different iterator types to be unified into a single type,
/// enabling heterogeneous collections where different rows may come from different
/// concrete iterator types. The Either type implements Iterator when both variants
/// yield the same item type.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R, T> Iterator for Either<L, R>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.next(),
            Either::Right(r) => r.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Either::Left(l) => l.size_hint(),
            Either::Right(r) => r.size_hint(),
        }
    }
}

impl<L, R, T> ExactSizeIterator for Either<L, R>
where
    L: ExactSizeIterator<Item = T>,
    R: ExactSizeIterator<Item = T>,
{
    #[inline]
    fn len(&self) -> usize {
        match self {
            Either::Left(l) => l.len(),
            Either::Right(r) => r.len(),
        }
    }
}

impl<L, R, T> DoubleEndedIterator for Either<L, R>
where
    L: DoubleEndedIterator<Item = T>,
    R: DoubleEndedIterator<Item = T>,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.next_back(),
            Either::Right(r) => r.next_back(),
        }
    }
}

// nth_back has a default impl, so no need to override.

impl<L, R, T> std::iter::FusedIterator for Either<L, R>
where
    L: std::iter::FusedIterator<Item = T>,
    R: std::iter::FusedIterator<Item = T>,
{
}
