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

pub fn handle_derive(input: TokenStream) -> TokenStream {
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
        impl #impl_generics itemize_2::IntoRows<#ident #ty_generics> for #ident #ty_generics #where_clause {
            type RowIter = std::iter::Once<#ident #ty_generics>;
            type Rows = std::iter::Once<Self::RowIter>;
            fn into_rows(self) -> Self::Rows {
                std::iter::once(std::iter::once(self))
            }
        }

        // #from_type_impls

        // #from_tuples_impls

        // #from_collections_impls
    };
    TokenStream::from(expanded)
}
