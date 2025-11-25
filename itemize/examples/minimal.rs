use itemize::{IntoItems, IntoRows};

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

#[derive(Debug, IntoItems, IntoRows)]
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
    let single = collect_counts(5);
    assert_eq!(single, vec![5]);

    let grid = collect_rows((vec!["a", "b"], ["c", "d", "e"]));
    assert_eq!(grid, vec![vec!["a", "b"], vec!["c", "d", "e"]]);
}
