pub trait IntoItems<Item> {
    type IntoIter: ::std::iter::Iterator<Item = Item>;
    fn into_items(self) -> Self::IntoIter;
}

pub trait IntoRows<Row> {
    type RowIter: ::std::iter::Iterator<Item = Row>;
    type Rows: ::std::iter::Iterator<Item = Self::RowIter>;
    fn into_rows(self) -> Self::Rows;
}

pub struct MySimpleType(String);

impl<T> From<T> for MySimpleType
where
    T: std::fmt::Display,
{
    fn from(value: T) -> Self {
        MySimpleType(value.to_string())
    }
}

impl IntoRows<MySimpleType> for &str {
    type RowIter = std::iter::Once<MySimpleType>;
    type Rows = std::iter::Once<Self::RowIter>;
    fn into_rows(self) -> Self::Rows {
        std::iter::once(std::iter::once(MySimpleType::from(self)))
    }
}
