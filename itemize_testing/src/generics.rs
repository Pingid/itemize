#![allow(warnings)]

use itemize::*;

#[derive(IntoItems, IntoRows)]
#[items_from(types(String, char, &'a str), tuples(2), collections(vec, slice, array))]
pub struct MySimpleType<T>(T)
where
    T: Clone;

impl<T> From<T> for MySimpleType<String>
where
    String: From<T>,
{
    fn from(value: T) -> Self {
        MySimpleType(String::from(value))
    }
}

#[test]
fn test_into_items() {
    fn into_items(x: impl IntoItems<MySimpleType<String>>) -> Vec<MySimpleType<String>> {
        x.into_items().collect()
    }

    let _ = into_items("hello");
    let _ = into_items('a');
    let _ = into_items(("1",));
    let _ = into_items(("1", "2"));
    let _ = into_items(vec!["4", "5", "6"]);
    // let _ = into_items(&vec!["4", "5", "6"]);
    let _ = into_items(["a", "b", "c"]);

    fn into_rows(x: impl IntoRows<MySimpleType<String>>) -> Vec<Vec<MySimpleType<String>>> {
        x.into_rows().map(|row| row.collect()).collect()
    }
    let _ = into_rows(MySimpleType("hello".to_string()));
    let _ = into_rows((("a", "b"), ("c", "d")));
    let _ = into_rows([["a", "b", "c"], ["d", "e", "f"]]);
    let _ = into_rows(vec![["a", "b", "c"], ["d", "e", "f"]]);
    let _ = into_rows(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
    // let _ = into_rows(vec![&vec!["a", "b", "c"], &vec!["d", "e", "f"]]);
}
