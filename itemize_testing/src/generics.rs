#![allow(warnings)]

use itemize::IntoItems;
use itemize_derive::*;

#[derive(IntoItems, IntoRows)]
#[items_from(
    types(String, char),
    tuples(2),
    collections(Vec<T>, &[T], [T; N])
)]
pub struct MySimpleType<T>(T);

impl<T> From<T> for MySimpleType<String>
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        MySimpleType(String::from(value))
    }
}

// impl<T, U> itemize_2::IntoRows<MySimpleType<T>> for Vec<U>
// where
//     U: IntoItems<MySimpleType<T>>,
// {
//     type RowIter = <U as IntoItems<MySimpleType<T>>>::IntoIter;
//     type Rows = std::iter::Map<<U as IntoItems<MySimpleType<T>>>::IntoIter, fn(U) -> Self::RowIter>;
//     fn into_rows(self) -> Self::Rows {
//         self.into_iter().map(|item| item.into_items())
//     }
// }

#[test]
fn test_into_items() {
    fn into_items(x: impl itemize_2::IntoItems<MySimpleType<String>>) -> Vec<MySimpleType<String>> {
        x.into_items().collect()
    }

    let _ = into_items(String::from("hello"));
    let _ = into_items('a');
    let _ = into_items(("1",));
    let _ = into_items(("1", "2"));
    let _ = into_items(vec!["4", "5", "6"]);
    let _ = into_items(["a", "b", "c"]);

    fn into_rows(
        x: impl itemize_2::IntoRows<MySimpleType<String>>,
    ) -> Vec<Vec<MySimpleType<String>>> {
        x.into_rows().map(|row| row.collect()).collect()
    }
    let _ = into_rows(MySimpleType("hello".to_string()));
    // let _ = into_rows("hello".to_string());
    // let _ = into_rows((('a', 'b'), ('c', 'd')));
    // let _ = into_rows([["a", "b", "c"], ["d", "e", "f"]]);
}
