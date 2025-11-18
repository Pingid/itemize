// pub struct MySimpleType(String);

// impl<T> From<T> for MySimpleType
// where
//     T: std::fmt::Display,
// {
//     fn from(value: T) -> Self {
//         MySimpleType(value.to_string())
//     }
// }

// pub trait IntoItemsMySimpleType {
//     type IntoIter: ::std::iter::Iterator<Item = MySimpleType>;
//     fn into_items(self) -> Self::IntoIter;
// }
// impl IntoItemsMySimpleType for &str {
//     type IntoIter = std::iter::Once<MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         std::iter::once(MySimpleType::from(self))
//     }
// }
// impl IntoItemsMySimpleType for String {
//     type IntoIter = std::iter::Once<MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         std::iter::once(MySimpleType::from(self))
//     }
// }
// impl IntoItemsMySimpleType for i32 {
//     type IntoIter = std::iter::Once<MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         std::iter::once(MySimpleType::from(self))
//     }
// }
// impl IntoItemsMySimpleType for f64 {
//     type IntoIter = std::iter::Once<MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         std::iter::once(MySimpleType::from(self))
//     }
// }
// impl<A0> IntoItemsMySimpleType for (A0,)
// where
//     MySimpleType: From<A0>,
// {
//     type IntoIter = std::array::IntoIter<MySimpleType, 1usize>;
//     fn into_items(self) -> Self::IntoIter {
//         let (a0,) = self;
//         <[MySimpleType; 1]>::into_iter([MySimpleType::from(a0)])
//     }
// }
// impl<A0, A1> IntoItemsMySimpleType for (A0, A1)
// where
//     MySimpleType: From<A0>,
//     MySimpleType: From<A1>,
// {
//     type IntoIter = std::array::IntoIter<MySimpleType, 2usize>;
//     fn into_items(self) -> Self::IntoIter {
//         let (a0, a1) = self;
//         <[MySimpleType; 2]>::into_iter([MySimpleType::from(a0), MySimpleType::from(a1)])
//     }
// }

// impl<T> IntoItemsMySimpleType for Vec<T>
// where
//     MySimpleType: From<T>,
// {
//     type IntoIter = std::vec::IntoIter<MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         self.into_iter()
//             .map(MySimpleType::from)
//             .collect::<Vec<_>>()
//             .into_iter()
//     }
// }

// impl<'a, T> IntoItemsMySimpleType for &'a [T]
// where
//     MySimpleType: From<&'a T>,
// {
//     type IntoIter = std::iter::Map<std::slice::Iter<'a, T>, fn(&'a T) -> MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         self.iter().map(MySimpleType::from as fn(&'a T) -> MySimpleType)
//     }
// }

// impl<T, const N: usize> IntoItemsMySimpleType for [T; N]
// where
//     MySimpleType: From<T>,
// {
//     type IntoIter = std::iter::Map<std::array::IntoIter<T, N>, fn(T) -> MySimpleType>;
//     fn into_items(self) -> Self::IntoIter {
//         IntoIterator::into_iter(self).map(MySimpleType::from as fn(T) -> MySimpleType)
//     }
// }

// #[test]
// fn test_into_items() {
//     fn into_items(x: impl IntoItemsMySimpleType) -> Vec<MySimpleType> {
//         x.into_items().collect()
//     }
//     let _ = into_items(1);
//     let _ = into_items("1");
//     let _ = into_items(("1", "2"));
//     let _ = into_items(vec!["4", "5", "6"]); // TODO: implement from_collections
//     let _ = into_items([1, 2, 3]);
// }
