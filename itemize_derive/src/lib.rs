use proc_macro::TokenStream;

#[proc_macro_derive(IntoItems, attributes(items_from))]
pub fn derive_into_items(input: TokenStream) -> TokenStream {
    itemize_derive_impl::handle_derive_into_items(input.into()).into()
}

#[proc_macro_derive(IntoRows, attributes(items_from))]
pub fn derive_into_rows(input: TokenStream) -> TokenStream {
    itemize_derive_impl::handle_derive_into_rows(input.into()).into()
}

#[proc_macro_derive(TryIntoItems, attributes(items_from))]
pub fn derive_try_into_items(input: TokenStream) -> TokenStream {
    itemize_derive_impl::handle_derive_try_into_items(input.into()).into()
}

#[proc_macro_derive(TryIntoRows, attributes(items_from))]
pub fn derive_try_into_rows(input: TokenStream) -> TokenStream {
    itemize_derive_impl::handle_derive_try_into_rows(input.into()).into()
}
