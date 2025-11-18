/// A trait for types that can produce an iterator of their items.
///
/// This is an internal trait used to enable blanket implementations
/// of `IntoItems` and `IntoRows` without coherence conflicts.
pub trait ItemsFromIter {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn items_from_iter(self) -> Self::IntoIter;
}

impl<T> ItemsFromIter for Option<T> {
    type Item = T;
    type IntoIter = std::option::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T, E> ItemsFromIter for Result<T, E> {
    type Item = T;
    type IntoIter = std::result::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> ItemsFromIter for Vec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> ItemsFromIter for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> ItemsFromIter for &'a [T] {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, const N: usize> ItemsFromIter for [T; N] {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T, const N: usize> ItemsFromIter for &'a [T; N] {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ItemsFromIter for std::collections::HashSet<T> {
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> ItemsFromIter for &'a std::collections::HashSet<T> {
    type Item = &'a T;
    type IntoIter = std::collections::hash_set::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> ItemsFromIter for std::collections::HashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, K, V> ItemsFromIter for &'a std::collections::HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::hash_map::Iter<'a, K, V>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ItemsFromIter for std::collections::BTreeSet<T> {
    type Item = T;
    type IntoIter = std::collections::btree_set::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> ItemsFromIter for &'a std::collections::BTreeSet<T> {
    type Item = &'a T;
    type IntoIter = std::collections::btree_set::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> ItemsFromIter for std::collections::BTreeMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::btree_map::IntoIter<K, V>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, K, V> ItemsFromIter for &'a std::collections::BTreeMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::collections::btree_map::Iter<'a, K, V>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ItemsFromIter for std::collections::VecDeque<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> ItemsFromIter for &'a std::collections::VecDeque<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ItemsFromIter for std::collections::LinkedList<T> {
    type Item = T;
    type IntoIter = std::collections::linked_list::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> ItemsFromIter for &'a std::collections::LinkedList<T> {
    type Item = &'a T;
    type IntoIter = std::collections::linked_list::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ItemsFromIter for std::collections::BinaryHeap<T> {
    type Item = T;
    type IntoIter = std::collections::binary_heap::IntoIter<T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> ItemsFromIter for &'a std::collections::BinaryHeap<T> {
    type Item = &'a T;
    type IntoIter = std::collections::binary_heap::Iter<'a, T>;
    fn items_from_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> ItemsFromIter for Box<T>
where
    T: ItemsFromIter,
{
    type Item = T::Item;
    type IntoIter = T::IntoIter;
    fn items_from_iter(self) -> Self::IntoIter {
        T::items_from_iter(*self)
    }
}
