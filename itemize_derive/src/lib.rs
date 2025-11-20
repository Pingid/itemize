use proc_macro::TokenStream;

mod into_items;

#[proc_macro_derive(IntoItems, attributes(into_items))]
pub fn derive_into_items(input: TokenStream) -> TokenStream {
    into_items::handle_derive_into_items(input)
}
