use proc_macro::TokenStream;

mod into_items;
mod into_rows;

#[proc_macro_derive(IntoItems, attributes(items_from))]
pub fn derive_into_items(input: TokenStream) -> TokenStream {
    into_items::handle_derive(input)
}

#[proc_macro_derive(IntoRows, attributes(items_from))]
pub fn derive_into_rows(input: TokenStream) -> TokenStream {
    into_rows::handle_derive(input)
}
