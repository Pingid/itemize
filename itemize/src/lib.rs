#[cfg(feature = "derive")]
pub use itemize_derive::*;

pub mod either;
pub use either::Either;

pub trait IntoItems<Item> {
    type IntoIter: ::std::iter::Iterator<Item = Item>;
    fn into_items(self) -> Self::IntoIter;
}

pub trait TryIntoItems<Item, E> {
    type IntoIter: ::std::iter::Iterator<Item = Result<Item, E>>;
    fn try_into_items(self) -> Self::IntoIter;
}

pub trait IntoRows<Row> {
    type RowIter: ::std::iter::Iterator<Item = Row>;
    type Rows: ::std::iter::Iterator<Item = Self::RowIter>;
    fn into_rows(self) -> Self::Rows;
}

pub trait TryIntoRows<Row, E> {
    type RowIter: ::std::iter::Iterator<Item = Result<Row, E>>;
    type Rows: ::std::iter::Iterator<Item = Self::RowIter>;
    fn try_into_rows(self) -> Self::Rows;
}
