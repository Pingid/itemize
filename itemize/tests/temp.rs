use itemize::*;

#[derive(IntoRows)]
// #[items_from(types(String, char, &'a str), tuples(2), collections(vec, slice, array))]
// #[items_from(types(&'a str, String))]
#[items_from(tuples(2))]
pub struct Foo<T>(T)
where
    T: Clone;

impl<T> From<T> for Foo<String>
where
    T: std::fmt::Display,
{
    fn from(value: T) -> Self {
        Foo(value.to_string())
    }
}

// impl<T, A0, A1> IntoVariadicRows<T> for (A0, A1)
// where
//     A0: IntoItems<T>,
//     A1: IntoItems<T>,
// {
//     type RowIter = Either<A0::IntoIter, A1::IntoIter>;
//     type Rows = std::array::IntoIter<Self::RowIter, 2>;
//     fn into_variadic_rows(self) -> Self::Rows {
//         let (a0, a1) = self;
//         use Either::{Left, Right};
//         [Left(a0.into_items()), Right(a1.into_items())].into_iter()
//     }
// }

// impl < T, _Item0 > itemize :: IntoItems < Foo < T > > for (_Item0,) where T :
// Clone, Foo < T > : :: std :: convert :: From < _Item0 > ,
// {
//     type IntoIter = std :: array :: IntoIter < Foo < T > , 1usize > ;
//     fn into_items(self) -> Self :: IntoIter
//     { let (a0,) = self; [< Foo < T > :: from(a0)].into_iter() }
// }

// impl<T, _Item0> itemize::IntoItems<Foo<T>> for (_Item0,)
// where
//     T: Clone,
//     Foo<T>: ::std::convert::From<_Item0>,
// {
//     type IntoIter = ::std::iter::Once<Foo<T>>;
//     fn into_items(self) -> Self::IntoIter {
//         ::std::iter::once(<Foo<T> as ::std::convert::From<_Item0>>::from(self.0))
//     }
// }

// impl<T, _Item0, _Item1> itemize::IntoItems<Foo<T>> for (_Item0, _Item1)
// where
//     T: Clone,
//     Foo<T>: ::std::convert::From<_Item0>,
//     Foo<T>: ::std::convert::From<_Item1>,
// {
//     type IntoIter = Either<::std::iter::Once<Foo<T>>, ::std::iter::Once<Foo<T>>>;
//     fn into_items(self) -> Self::IntoIter {
//         unimplemented!()
//     }
// }

// impl<'a, T> itemize::IntoItems<Foo<T>> for &'a str
// where
//     T: Clone,
//     Foo<T>: ::std::convert::From<&'a str>,
// {
//     type IntoIter = ::std::iter::Once<Foo<T>>;
//     fn into_items(self) -> Self::IntoIter {
//         ::std::iter::once(<Foo<T> as ::std::convert::From<&'a str>>::from(self))
//     }
// }

// impl<'a, __Item, T> itemize::IntoItems<Foo<T>> for &'a [__Item]
// where
//     T: Clone,
//     Foo<T>: ::std::convert::From<__Item>,
// {
//     type IntoIter = ::std::iter::Map<::std::slice::IntoIter<__Item>, fn(__Item) -> Foo<T>>;
//     fn into_items(self) -> Self::IntoIter {
//         fn map_item<Target, Item>(item: Item) -> Target
//         where
//             Target: ::std::convert::From<Item>,
//         {
//             Target::from(item)
//         }
//         self.into_iter().map(map_item::<Foo<T>, __Item>)
//     }
// }

// #[derive(IntoItems)]
// #[items_from(types(String, char, &'a str), tuples(2), collections(vec, slice, array))]
// pub struct Foo(String);

// impl<T> From<T> for Foo
// where
//     T: std::fmt::Display,
// {
//     fn from(value: T) -> Self {
//         Foo(value.to_string())
//     }
// }

// #[test]
// fn test_into_items() {
//     fn into_items(x: impl IntoItems<Foo<String>>) -> Vec<Foo<String>> {
//         x.into_items().collect()
//     }
//     let _ = into_items("hello");
//     let _ = into_items('a');
//     let _ = into_items(("1",));
//     let _ = into_items(("10", 10));
//     let _ = into_items(vec!["4", "5", "6"]);
//     let _ = into_items(["a", "b", "c"]);
// }
