use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod collections;
mod context;
mod tuples;
mod types;

use collections::from_collections;
use context::Context;
use tuples::from_tuples;
use types::from_type;

pub fn handle_derive_into_items(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let context = match Context::try_new(&ast) {
        Ok(ctx) => ctx,
        Err(e) => return e.to_compile_error().into(),
    };
    let impl_generics = &context.impl_generics;
    let ty_generics = &context.ty_generics;
    let where_clause = &context.where_clause;
    let ident = &context.ident;

    let from_type_impls = from_type(&context);
    let from_tuples_impls = from_tuples(&context);
    let from_collections_impls = from_collections(&context);

    let expanded = quote! {
        impl #impl_generics itemize_2::IntoItems<#ident #ty_generics> for #ident #ty_generics #where_clause {
            type IntoIter = ::std::iter::Once<#ident #ty_generics>;
            fn into_items(self) -> Self::IntoIter {
                ::std::iter::once(self)
            }
        }

        #from_type_impls

        #from_tuples_impls

        #from_collections_impls
    };
    TokenStream::from(expanded)
}
