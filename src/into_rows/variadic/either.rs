/// Either type for composing heterogeneous iterators through nesting.
///
/// This enum allows two different iterator types to be unified into a single type,
/// enabling heterogeneous collections where different rows may come from different
/// concrete iterator types. The Either type implements Iterator when both variants
/// yield the same item type.
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Implements Iterator for Either, delegating to the contained iterator.
/// Both variants must yield the same item type.
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

    #[inline]
    fn count(self) -> usize {
        match self {
            Either::Left(l) => l.count(),
            Either::Right(r) => r.count(),
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.last(),
            Either::Right(r) => r.last(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.nth(n),
            Either::Right(r) => r.nth(n),
        }
    }

    #[inline]
    fn collect<B>(self) -> B
    where
        B: std::iter::FromIterator<Self::Item>,
    {
        match self {
            Either::Left(l) => l.collect(),
            Either::Right(r) => r.collect(),
        }
    }

    #[inline]
    fn all<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        match self {
            Either::Left(l) => l.all(f),
            Either::Right(r) => r.all(f),
        }
    }

    #[inline]
    fn any<F>(&mut self, f: F) -> bool
    where
        F: FnMut(Self::Item) -> bool,
    {
        match self {
            Either::Left(l) => l.any(f),
            Either::Right(r) => r.any(f),
        }
    }

    #[inline]
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        match self {
            Either::Left(l) => l.find(predicate),
            Either::Right(r) => r.find(predicate),
        }
    }

    #[inline]
    fn position<P>(&mut self, predicate: P) -> Option<usize>
    where
        P: FnMut(Self::Item) -> bool,
    {
        match self {
            Either::Left(l) => l.position(predicate),
            Either::Right(r) => r.position(predicate),
        }
    }

    #[inline]
    fn for_each<F>(self, f: F)
    where
        F: FnMut(Self::Item),
    {
        match self {
            Either::Left(l) => l.for_each(f),
            Either::Right(r) => r.for_each(f),
        }
    }
}

/// Implements ExactSizeIterator when both variants are exact-size.
/// Delegates len() to the contained iterator.
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

/// Implements DoubleEndedIterator when both variants are double-ended.
/// Allows iteration from both ends.
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

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        match self {
            Either::Left(l) => l.nth_back(n),
            Either::Right(r) => r.nth_back(n),
        }
    }
}

/// Implements FusedIterator when both variants are fused.
/// Guarantees the iterator will continue returning None after exhaustion.
impl<L, R, T> std::iter::FusedIterator for Either<L, R>
where
    L: std::iter::FusedIterator<Item = T>,
    R: std::iter::FusedIterator<Item = T>,
{
}
