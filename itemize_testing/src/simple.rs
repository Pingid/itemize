#![allow(warnings)]

use itemize_derive::*;

#[derive(IntoItems)]
#[into_items(
    from_types(&str, String, i32, f64),
    from_tuples(2),
    from_collections(Vec<T>, &[T], [T; N])
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
    fn into_items(x: impl itemize_2::IntoItems<MySimpleType>) -> Vec<MySimpleType> {
        x.into_items().collect()
    }
    let _ = into_items(1);
    let _ = into_items("1");
    let _ = into_items(("1",));
    let _ = into_items(("1", "2"));
    let _ = into_items(vec!["4", "5", "6"]);
    let _ = into_items([1, 2, 3]);
}
