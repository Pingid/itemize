use itemize::*;

#[derive(IntoItems, IntoRows)]
#[items_from(types(String, char, &'a str), tuples(2), collections(vec, slice, array))]
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

#[test]
fn test_into_items() {
    fn into_items(x: impl IntoItems<Foo<String>>) -> Vec<Foo<String>> {
        x.into_items().collect()
    }
    let _ = into_items("hello");
    let _ = into_items('a');
    let _ = into_items(("1",));
    let _ = into_items(("10", 10));
    let _ = into_items(vec!["4", "5", "6"]);
    let _ = into_items(["a", "b", "c"]);
}

#[test]
fn test_into_rows() {
    fn into_rows(x: impl IntoRows<Foo<String>>) -> Vec<Vec<Foo<String>>> {
        x.into_rows().map(|row| row.collect()).collect()
    }
    let _ = into_rows(("hello",));
    let _ = into_rows((("a", "b"), ("c", "d")));
    let _ = into_rows([["a", "b", "c"], ["d", "e", "f"]]);
    let _ = into_rows(vec![["a", "b", "c"], ["d", "e", "f"]]);
    let _ = into_rows(vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
}

#[derive(TryIntoItems, TryIntoRows)]
#[items_from(types(String, char, &'a str), tuples(2), collections(vec, slice, array))]
pub struct Bar<T>(T)
where
    T: Clone;

impl TryFrom<String> for Bar<usize> {
    type Error = std::num::ParseIntError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse::<usize>().map(Bar)
    }
}

impl TryFrom<&str> for Bar<usize> {
    type Error = std::num::ParseIntError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<usize>().map(Bar)
    }
}

impl TryFrom<usize> for Bar<usize> {
    type Error = std::num::ParseIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(Bar(value))
    }
}

#[test]
fn test_try_into_items() {
    fn try_into_items(
        x: impl TryIntoItems<Bar<usize>, std::num::ParseIntError>,
    ) -> Result<Vec<Bar<usize>>, std::num::ParseIntError> {
        x.try_into_items().collect()
    }
    let _ = try_into_items("hello");
    let _ = try_into_items("a");
    let _ = try_into_items(("1",));
    let _ = try_into_items((10, "20".to_string()));
    let _ = try_into_items(vec!["4", "5", "6"]);
    let _ = try_into_items(["a", "b", "c"]);
}

#[test]
fn test_try_into_rows() {
    fn try_into_rows(
        x: impl TryIntoRows<Bar<usize>, std::num::ParseIntError>,
    ) -> Result<Vec<Vec<Bar<usize>>>, std::num::ParseIntError> {
        x.try_into_rows().map(|row| row.collect()).collect()
    }
    let _ = try_into_rows(("hello",));
    let _ = try_into_rows((("a", "b".to_string()), (3, "d")));
    let _ = try_into_rows([["a", "b", "c"], ["d", "e", "f"]]);
    let _ = try_into_rows(vec![["a", "b", "c"], ["d", "e", "f"]]);
    let _ = try_into_rows(vec![vec!["a", "b", "c"], vec!["1", "2"]]);
}
