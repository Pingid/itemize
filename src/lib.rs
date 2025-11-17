pub(crate) mod iter;

mod into_items;
pub use into_items::*;

#[cfg(feature = "into_rows")]
mod into_rows;
#[cfg(feature = "into_rows")]
pub use into_rows::*;

#[macro_export]
macro_rules! derive_item_conversions {
    ($for:ty => $lifetime:lifetime $type:ty $(, $($rest:tt)*)?) => {
        impl<$lifetime> $crate::into_items::IntoItems<&$lifetime $for> for &$lifetime $type {
            type IntoIter = std::iter::Once<&$lifetime $for>;
            fn into_items(self) -> Self::IntoIter {
                std::iter::once(self.into())
            }
        }

        #[cfg(feature = "into_rows")]
        impl<$lifetime> $crate::into_rows::IntoRows<&$lifetime $for> for &$lifetime $type {
            type RowIter = std::iter::Once<&$lifetime $for>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self.into()))
            }
        }

        #[cfg(feature = "into_rows_variadic")]
        impl<$lifetime> $crate::into_rows::IntoVariadicRows<&$lifetime $for> for &$lifetime $type {
            type RowIter = std::iter::Once<&$lifetime $for>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_variadic_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self.into()))
            }
        }

        $($crate::derive_item_conversions!($for => $($rest)*);)?
    };
    ($for:ty => $type:ty $(, $($rest:tt)*)?) => {
        impl $crate::into_items::IntoItems<$for> for $type {
            type IntoIter = std::iter::Once<$for>;
            fn into_items(self) -> Self::IntoIter {
                std::iter::once(self.into())
            }
        }

        #[cfg(feature = "into_rows")]
        impl $crate::into_rows::IntoRows<$for> for $type {
            type RowIter = std::iter::Once<$for>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self.into()))
            }
        }

        #[cfg(feature = "into_rows_variadic")]
        impl $crate::into_rows::IntoVariadicRows<$for> for $type {
            type RowIter = std::iter::Once<$for>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_variadic_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self.into()))
            }
        }

        $($crate::derive_item_conversions!($for => $($rest)*);)?
    };
}

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

    derive_item_conversions!(Custom => 'a Custom, usize, f64);

    fn into_items(x: impl IntoItems<Custom>) -> Vec<Custom> {
        x.into_items().collect()
    }

    #[test]
    fn test_into_items_for_collections() {
        into_items([1, 2, 3]);
        into_items(&[1, 2, 3]);
        into_items(vec![1, 2, 3]);
        into_items(&vec![1, 2, 3]);
        into_items(HashSet::from([1, 2, 3]));
        into_items(&HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_into_items_for_tuples() {
        into_items((1, "2", 3f64));
        into_items(&(1, "2", 3f64));
        into_items(&(1, &"2", 3f64));
    }

    #[test]
    fn test_into_items_for_single() {
        into_items(1);
        into_items(3.14f64);
    }

    fn into_rows(x: impl IntoRows<Custom>) -> Vec<Vec<Custom>> {
        x.into_rows().map(|row| row.collect()).collect()
    }

    #[test]
    fn test_into_rows_for_collections() {
        into_rows(vec![&vec![1, 2, 3], &vec![4, 5, 6]]);
        into_rows(&vec![[1, 2, 3], [4, 5, 6]]);
    }

    #[test]
    fn test_into_rows_for_tuples() {
        into_rows(((1, "2"), (4, "5")));
        into_rows([(1, "2"), (4, "5")]);
        into_rows([&(1, "2"), &(4, "5")]);
        into_rows(&[(1, "2"), (4, "5")]);
        into_rows(&[&(1, "2"), &(4, "5")]);
    }

    #[test]
    fn test_into_rows_for_single() {
        into_rows(1);
        into_rows(3.14f64);
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
