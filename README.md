# itemize

`itemize` lets you write APIs that accept tuples, collections, or single values while receiving a predictable iterator shape inside your function.

## Core traits

- `IntoItems<T>` turns 1D inputs into an iterator of `T`.
- `IntoRows<T>` lifts nested inputs into iterators of rows; enable `hetero` to mix lengths, or `dynamic` / `variadic` for alternate strategies.
- `impl_into_items!` and `impl_into_rows!` opt single-value types into the traits.

## Example

```rust
use itemize::IntoItems;

struct UserId(u64);

impl From<usize> for UserId {
    fn from(id: usize) -> Self {
        UserId(id as u64)
    }
}

impl From<i32> for UserId {
    fn from(id: i32) -> Self {
        UserId(id as u64)
    }
}

fn fetch(ids: impl IntoItems<UserId>) -> Vec<UserId> {
    ids.into_items().collect()
}

fetch([1, 2, 3]);
fetch(vec![1, 2, 3]);
fetch((4i32, 5, 6));
```

To support single arguments use macro with types that impl Into<UserId>

```rust
derive_item_conversions!(UserId => u64, usize, i32)

fetch(1usize)
fetch(1i32)
```

To collect 2D inputs use `IntoRows`:

```rust
use itemize::IntoRows;

fn matrix(data: impl IntoRows<f64>) -> Vec<Vec<f64>> {
    data.into_rows().map(|row| row.collect()).collect()
}

let rows = matrix(((1.0, 2.0), (3.0, 4.0)));
```

To collect 2D inputs into iter of variadic iterators use `IntoVariadicRows`:

```rust
use itemize::IntoVariadicRows;

fn matrix(data: impl IntoVariadicRows<f64>) -> Vec<Vec<f64>> {
    data.into_rows().map(|row| row.collect()).collect()
}

let rows = matrix(((1.0, 2.0), (3.0, 4.0, 5.0)));
```

## Installation

```toml
[dependencies]
itemize = "0.1"
```

Feature flags:

- `into_items` – enable `IntoItems`.
- `into_rows` – enable `IntoRows`.
- `variadic` – enable `IntoVariadicRows`.

Tuple implementations are generated up to `INTO_VEC_MAX_TUPLE_SIZE` (default 12). Override the limit by setting that environment variable at build time.
