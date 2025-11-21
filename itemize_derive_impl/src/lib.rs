use codegen::generate_derive;
use proc_macro2::TokenStream;
use trait_config::{TraitConfig, TraitKind, TraitVariant};

mod codegen;
mod context;
mod trait_config;
mod unified;
mod util;

pub fn handle_derive_into_items(input: TokenStream) -> TokenStream {
    let config = TraitConfig::new(TraitVariant::Regular, TraitKind::Items);
    generate_derive(input, config)
}

pub fn handle_derive_into_rows(input: TokenStream) -> TokenStream {
    let config = TraitConfig::new(TraitVariant::Regular, TraitKind::Rows);
    generate_derive(input, config)
}

pub fn handle_derive_try_into_items(input: TokenStream) -> TokenStream {
    let config = TraitConfig::new(TraitVariant::Try, TraitKind::Items);
    generate_derive(input, config)
}

pub fn handle_derive_try_into_rows(input: TokenStream) -> TokenStream {
    let config = TraitConfig::new(TraitVariant::Try, TraitKind::Rows);
    generate_derive(input, config)
}
