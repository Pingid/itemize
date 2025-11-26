use itemize::TryIntoItems;

#[derive(Debug)]
struct ParseError(#[allow(dead_code)] String);

impl From<std::num::ParseIntError> for ParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        ParseError(e.to_string())
    }
}

#[derive(TryIntoItems)]
#[items_from(
    types(String, &'a str, i64),
    tuples(3),
    collections(vec, slice, array),
    error_type(ParseError)
)]
struct Int(#[allow(dead_code)] i64);

impl TryFrom<&str> for Int {
    type Error = std::num::ParseIntError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse::<i64>().map(Int)
    }
}

impl TryFrom<String> for Int {
    type Error = std::num::ParseIntError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<i64>().map(Int)
    }
}

impl TryFrom<i64> for Int {
    type Error = std::num::ParseIntError;
    fn try_from(n: i64) -> Result<Self, Self::Error> {
        Ok(Int(n))
    }
}

fn parse_ints(input: impl TryIntoItems<Int, ParseError>) -> Result<Vec<Int>, ParseError> {
    input.try_into_items().collect()
}

fn main() -> Result<(), ParseError> {
    // single value
    let _ = parse_ints("42")?;

    // tuple of mixed types
    let _ = parse_ints(("1", "2".to_string(), 3))?;

    // array
    let _ = parse_ints(["10", "20", "30"])?;

    // vec
    let _ = parse_ints(vec!["100", "200"])?;

    Ok(())
}
