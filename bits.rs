#[macro_export]
macro_rules! derive_from_conversions {
    ($for:ty => $lifetime:lifetime $type:ty $(, $($rest:tt)*)?) => {
        impl<$lifetime> $crate::into_items::IntoItems<$for> for &$lifetime $type {
            type IntoIter = std::iter::Once<$for>;
            fn into_items(self) -> Self::IntoIter {
                std::iter::once(self.into())
            }
        }

        #[cfg(feature = "into_rows")]
        impl<$lifetime> $crate::into_rows::IntoRows<$for> for &$lifetime $type {
            type RowIter = std::iter::Once<$for>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self.into()))
            }
        }

        #[cfg(feature = "into_rows_variadic")]
        impl<$lifetime> $crate::into_rows::IntoVariadicRows<$for> for &$lifetime $type {
            type RowIter = std::iter::Once<$for>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_variadic_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self.into()))
            }
        }

        $($crate::derive_from_conversions!($for => $($rest)*);)?
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

        $($crate::derive_from_conversions!($for => $($rest)*);)?
    };
}

// #[macro_export]
// macro_rules! derive_try_from_conversions {
//     ($for:ty => &$lifetime:lifetime $type:ty $(, $($rest:tt)*)?) => {
//         impl<$lifetime, E> $crate::TryIntoItems<$for, E> for &$lifetime $type {
//             type IntoIter = std::iter::Once<Result<$for, E>>;
//             fn try_into_items(self) -> Self::IntoIter {
//                 std::iter::once(<$for>::try_from(self))
//             }
//         }

//         #[cfg(feature = "into_rows")]
//         impl<$lifetime, E> $crate::TryIntoRows<$for, E> for &$lifetime $type {
//             type RowIter = std::iter::Once<Result<$for, E>>;
//             type Rows = std::iter::Once<Self::RowIter>;
//             fn try_into_rows(self) -> Self::Rows {
//                 std::iter::once(std::iter::once(<$for>::try_from(self)))
//             }
//         }

//         #[cfg(feature = "into_rows_variadic")]
//         impl<$lifetime, E> $crate::TryIntoVariadicRows<$for, E> for &$lifetime $type {
//             type RowIter = std::iter::Once<Result<$for, E>>;
//             type Rows = std::iter::Once<Self::RowIter>;
//             fn try_into_variadic_rows(self) -> Self::Rows {
//                 std::iter::once(std::iter::once(<$for>::try_from(self)))
//             }
//         }

//         $($crate::derive_try_from_conversions!($for => $($rest)*);)?
//     };
//     ($for:ty => $type:ty $(, $($rest:tt)*)?) => {
//         impl<E> $crate::TryIntoItems<$for, E> for $type {
//             type IntoIter = std::iter::Once<Result<$for, E>>;
//             fn try_into_items(self) -> Self::IntoIter {
//                 std::iter::once(<$for>::try_from(self))
//             }
//         }

//         #[cfg(feature = "into_rows")]
//         impl<E> $crate::TryIntoRows<$for, E> for $type {
//             type RowIter = std::iter::Once<Result<$for, E>>;
//             type Rows = std::iter::Once<Self::RowIter>;
//             fn try_into_rows(self) -> Self::Rows {
//                 std::iter::once(std::iter::once(<$for>::try_from(self)))
//             }
//         }

//         #[cfg(feature = "into_rows_variadic")]
//         impl<E> $crate::TryIntoVariadicRows<$for, E> for $type {
//             type RowIter = std::iter::Once<Result<$for, E>>;
//             type Rows = std::iter::Once<Self::RowIter>;
//             fn try_into_variadic_rows(self) -> Self::Rows {
//                 std::iter::once(std::iter::once(<$for>::try_from(self)))
//             }
//         }

//         $($crate::derive_try_from_conversions!($for => $($rest)*);)?
//     };
// }
