#![allow(warnings)]

use itemize::*;

#[derive(IntoItems)]
#[items_from(
    types(&str, String, i32, f64),
    tuples(2),
    collections(vec, slice, array)
)]
pub struct MySimpleType(String);

impl<T> From<T> for MySimpleType
where
    T: std::fmt::Display,
{
    fn from(value: T) -> Self {
        MySimpleType(value.to_string())
    }
}

#[test]
fn test_into_items() {
    fn into_items(x: impl IntoItems<MySimpleType>) -> Vec<MySimpleType> {
        x.into_items().collect()
    }
    let _ = into_items(1);
    let _ = into_items("1");
    let _ = into_items(("1",));
    let _ = into_items(("1", "2"));
    let _ = into_items(vec!["4", "5", "6"]);
    let _ = into_items([1, 2, 3]);
}
