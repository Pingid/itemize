use codegen::generate_derive;
use proc_macro2::TokenStream;
use syn::DeriveInput;
use trait_config::{TraitConfig, TraitKind, TraitVariant};

use crate::context::Context;

mod codegen;
mod context;
mod generate;
mod trait_config;
mod unified;
mod util;

pub fn handle_derive_into_items(input: TokenStream) -> TokenStream {
    handle_generate(input, generate::into_items::generate)
    // let config = TraitConfig::new(TraitVariant::Regular, TraitKind::Items);
    // generate_derive(input, config)
}

pub fn handle_derive_into_rows(input: TokenStream) -> TokenStream {
    handle_generate(input, generate::into_rows::generate)
    // let config = TraitConfig::new(TraitVariant::Regular, TraitKind::Rows);
    // generate_derive(input, config)
}

pub fn handle_derive_try_into_items(input: TokenStream) -> TokenStream {
    handle_generate(input, generate::try_into_items::generate)
    // let config = TraitConfig::new(TraitVariant::Try, TraitKind::Items);
    // generate_derive(input, config)
}

pub fn handle_derive_try_into_rows(input: TokenStream) -> TokenStream {
    let config = TraitConfig::new(TraitVariant::Try, TraitKind::Rows);
    generate_derive(input, config)
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
