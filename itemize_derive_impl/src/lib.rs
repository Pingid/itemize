use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::context::Context;

mod context;
mod traits;
mod util;

pub fn handle_derive_into_items(input: TokenStream) -> TokenStream {
    handle_generate(input, traits::into_items::generate)
}

pub fn handle_derive_into_rows(input: TokenStream) -> TokenStream {
    handle_generate(input, traits::into_rows::generate)
}

pub fn handle_derive_try_into_items(input: TokenStream) -> TokenStream {
    handle_generate(input, traits::try_into_items::generate)
}

pub fn handle_derive_try_into_rows(input: TokenStream) -> TokenStream {
    handle_generate(input, traits::try_into_rows::generate)
}

fn handle_generate(
    input: TokenStream,
    generate: impl Fn(&Context<'_>) -> TokenStream,
) -> TokenStream {
    let ast = match syn::parse2::<DeriveInput>(input) {
        Ok(ast) => ast,
        Err(e) => return e.to_compile_error(),
    };
    let context = match Context::try_new(&ast) {
        Ok(ctx) => ctx,
        Err(e) => return e.to_compile_error(),
    };
    generate(&context)
}
