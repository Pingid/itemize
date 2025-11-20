use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::context::Context;

pub fn from_tuples(context: &Context) -> TokenStream {
    let Some(max) = context.attributes.tuples else {
        return TokenStream::new();
    };

    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let mut expanded = TokenStream::new();

    // Check if the struct is generic
    if context.generics.params.is_empty() {
        // Non-generic case
        for n in 1..=max {
            let type_params: Vec<_> = (0..n).map(|i| format_ident!("A{}", i)).collect();
            let vars: Vec<_> = (0..n).map(|i| format_ident!("a{}", i)).collect();
            let len = n;

            expanded.extend(quote! {
                impl<#(#type_params),*> itemize_2::IntoItems<#ident> for (#(#type_params,)*)
                where
                    #(#ident: From<#type_params>,)*
                {
                    type IntoIter = std::array::IntoIter<#ident, #len>;

                    fn into_items(self) -> Self::IntoIter {
                        let (#(#vars,)*) = self;
                        <[#ident; #len]>::into_iter([#(#ident::from(#vars)),*])
                    }
                }
            });
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        for n in 1..=max {
            let type_params: Vec<_> = (0..n).map(|i| format_ident!("__TupleItem{}", i)).collect();
            let vars: Vec<_> = (0..n).map(|i| format_ident!("a{}", i)).collect();
            let len = n;

            expanded.extend(quote! {
                impl<#(#type_params,)* #generic_params> itemize_2::IntoItems<#ident #ty_generics> for (#(#type_params,)*)
                #where_clause
                where
                    #(#ident #ty_generics: From<#type_params>,)*
                {
                    type IntoIter = std::array::IntoIter<#ident #ty_generics, #len>;

                    fn into_items(self) -> Self::IntoIter {
                        let (#(#vars,)*) = self;
                        <[#ident #ty_generics; #len]>::into_iter([#(#ident::from(#vars)),*])
                    }
                }
            });
        }
    }

    expanded
}
