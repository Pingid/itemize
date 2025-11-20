use proc_macro::TokenStream;

#[proc_macro_derive(IntoItems, attributes(items_from))]
pub fn derive_into_items(input: TokenStream) -> TokenStream {
    itemize_derive_impl::into_items::handle_derive(input.into()).into()
}

#[proc_macro_derive(IntoRows, attributes(items_from))]
pub fn derive_into_rows(input: TokenStream) -> TokenStream {
    itemize_derive_impl::into_rows::handle_derive(input.into()).into()
}
