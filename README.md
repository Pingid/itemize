# itemize

[![CI](https://github.com/Pingid/itemize/workflows/CI/badge.svg)](https://github.com/Pingid/itemize/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/itemize.svg)](https://crates.io/crates/itemize)
[![Documentation](https://docs.rs/itemize/badge.svg)](https://docs.rs/itemize)
[![License](https://img.shields.io/crates/l/itemize.svg)](https://github.com/Pingid/itemize#license)

Write APIs that accept single values, tuples, or collections—then work with a predictable iterator inside your function.

## Install

```bash
cargo add itemize
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
itemize = "0.1"
```

## Traits

- `IntoItems<T>` / `TryIntoItems<T, E>` – flatten inputs into `Iterator<Item = T>` (or `Result<T, E>`)
- `IntoRows<T>` / `TryIntoRows<T, E>` – flatten 2D inputs into nested iterators

### Trait bounds

- `IntoItems<T>` implementations expect `T: From<Source>` for every declared source type.
- `TryIntoItems<T, E>` implementations expect `T: TryFrom<Source, Error = SourceErr>` with `SourceErr: Into<E>`.

## Derive Macros

Generate trait implementations with derive macros and the `#[items_from(...)]` attribute:

- `types(...)` – types to accept (e.g., `String`, `&'a str`, `usize`)
- `tuples(n)` – support tuples up to size n
- `collections(vec, slice, array)` – which collection types to support
- `error_type(Type)` – lock `TryInto*` impls to a specific error type

## Examples

### `TryIntoItems` parsing

```rust
use itemize::TryIntoItems;

#[derive(Debug)]
struct ParseError(String);

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
struct Int(i64);

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
    let _ = parse_ints("42")?;
    let _ = parse_ints(("1", "2".to_string(), 3))?;
    let _ = parse_ints(["10", "20", "30"])?;
    let _ = parse_ints(vec!["100", "200"])?;
    Ok(())
}
```

### Minimal `IntoItems`

```rust
use itemize::IntoItems;

#[derive(Debug, IntoItems)]
#[items_from(types(u32), tuples(2), collections(vec, array))]
struct Count(u32);

impl From<u32> for Count {
    fn from(value: u32) -> Self {
        Count(value)
    }
}

fn collect_counts(input: impl IntoItems<Count>) -> Vec<u32> {
    input.into_items().map(|Count(n)| n).collect()
}

fn main() {
    let single = collect_counts(5);
    let mixed = collect_counts((vec![1, 2], [3, 4]));
    assert_eq!(single, vec![5]);
    assert_eq!(mixed, vec![1, 2, 3, 4]);
}
```

### Mixed-source `IntoRows`

```rust
use itemize::IntoRows;

#[derive(Debug, IntoRows)]
#[items_from(types(&'a str), tuples(2), collections(vec, array))]
struct Cell<'a>(&'a str);

impl<'a> From<&'a str> for Cell<'a> {
    fn from(value: &'a str) -> Self {
        Cell(value)
    }
}

fn collect_rows<'a>(input: impl IntoRows<Cell<'a>>) -> Vec<Vec<&'a str>> {
    input
        .into_rows()
        .map(|row| row.map(|Cell(value)| value).collect())
        .collect()
}

fn main() {
    let grid = collect_rows((vec!["a", "b"], ["c", "d", "e"]));
    assert_eq!(grid, vec![vec!["a", "b"], vec!["c", "d", "e"]]);
}
```

The tuple combines a `Vec` and an array, and `IntoRows` turns it into a `Vec<Vec<&str>>` with one row per source.

## License

MIT © [Dan Beaven](https://github.com/Pingid)
