# itemize

# itemize

`itemize` lets you write APIs that accept single values, tuples, or collections
(and even nested collections) while you work with a predictable iterator type
inside your function.

## Install

```bash
cargo add --git https://github.com/Pingid/itemize
```

## Library

- `IntoItems<T>` / `TryIntoItems<T, E>` – flatten 1D inputs into `Iterator<Item = T>`.
- `IntoRows<T>` / `TryIntoRows<T, E>` – flatten 2D inputs into `Iterator<Item = RowIter>`.
- `IntoVariadicRows<T>` / `TryIntoVariadicRows<T, E>` – like `IntoRows`, but rows can be
  different concrete collection types.

Common std types like `Vec`, arrays, slices, `Option`, `Result`, `HashSet`,
`BTreeMap`, `VecDeque`, `Box<…>`, etc. work out of the box.

## Example

```rust
use itemize::TryIntoItems;

#[derive(Debug)]
enum ParseError {
    BadInt(String),
}

#[derive(TryIntoItems)]
#[items_from(types(String, char, &'a str), tuples(3), collections(vec, slice, array))]
struct Answer(i64);

impl TryFrom<&str> for Answer {
    type Error = ParseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse::<i64>()
            .map(Answer)
            .map_err(|_| ParseError::BadInt(s.to_string()))
    }
}

impl TryFrom<f64> for Answer {
    type Error = ParseError;
    fn try_from(n: f64) -> Result<Self, Self::Error> {
        Ok(Answer(n as i64))
    }
}

impl TryFrom<usize> for Answer {
    type Error = ParseError;
    fn try_from(n: usize) -> Result<Self, Self::Error> {
        Ok(Answer(n as i64))
    }
}

fn parse_ints(inputs: impl TryIntoItems<Answer, ParseError>) -> Result<Vec<Answer>, ParseError> {
    inputs.try_into_items().collect()
}

fn parse_int_rows(
    inputs: impl TryIntoVariadicRows<Answer, ParseError>,
) -> Vec<Result<Vec<Answer>, ParseError>> {
    inputs
        .try_into_variadic_rows()
        .map(|rows| rows.collect())
        .collect()
}

fn demo() -> Result<(), ParseError> {
    // works with Vec<String>
    let values = parse_ints(["1", "2", "3"]);
    let more = parse_ints(("1", 2, 3.0))?;
    let single = parse_ints(("1",))?;
    let rows = parse_int_rows((("1", 2, 3.0), ("4", "5")));
    Ok(())
}
```
