#![allow(warnings)]

use itemize_derive::*;

#[derive(IntoItems)]
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
}
