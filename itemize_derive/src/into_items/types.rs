use proc_macro2::TokenStream;
use quote::quote;

use super::context::Context;

pub fn from_type(context: &Context) -> TokenStream {
    let impl_generics = &context.impl_generics;
    let ty_generics = &context.ty_generics;
    let where_clause = &context.where_clause;
    let ident = &context.ident;
    let from_types = &context.attributes.from_types;
    let expanded = quote! {
        #(
            impl #impl_generics itemize_2::IntoItems<#ident #ty_generics> for #from_types #where_clause {
                type IntoIter = ::std::iter::Once<#ident #ty_generics>;
                fn into_items(self) -> Self::IntoIter {
                    ::std::iter::once(#ident::from(self))
                }
            }
        )*
    };
    TokenStream::from(expanded)
}
