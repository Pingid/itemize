# itemize

# itemize

`itemize` lets you write APIs that accept single values, tuples, or collections
(and even nested collections) while you work with a predictable iterator type
inside your function.

## Install

```bash
cargo add --git https://github.com/Pingid/join
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
use itemize::{TryIntoItems, TryIntoVariadicRows};

#[derive(Debug)]
enum ParseError {
    BadInt(String),
}

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

## Feature flags

```toml
# default: everything on
itemize = { version = "0.1" }

# or be explicit
itemize = { version = "0.1", default-features = false, features = ["into_rows", "into_rows_variadic"] }
```

- `into_rows` – enables `IntoRows` / `TryIntoRows`.
- `into_rows_variadic` – enables `IntoVariadicRows` / `TryIntoVariadicRows` and the `Either` type.

`IntoItems` / `TryIntoItems` are always available.

---

## Tuple support

Tuple implementations for all traits are code-generated up to the arity given  
by the `INTO_VEC_MAX_TUPLE_SIZE` environment variable (default: `12`):

```bash
INTO_VEC_MAX_TUPLE_SIZE=8 cargo build
```

This affects tuples for `IntoItems`, `IntoRows` and `IntoVariadicRows`.
