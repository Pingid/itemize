use proc_macro2::TokenStream;
use quote::quote;

use super::context::Context;

pub fn from_type(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let from_types = &context.attributes.types;
    let ty_generics = &context.ty_generics;

    // Check if the struct is generic
    if context.generics.params.is_empty() {
        // Non-generic case
        let expanded = quote! {
            #(
                impl itemize_2::IntoItems<#ident> for #from_types {
                    type IntoIter = ::std::iter::Once<#ident>;
                    fn into_items(self) -> Self::IntoIter {
                        ::std::iter::once(#ident::from(self))
                    }
                }
            )*
        };
        TokenStream::from(expanded)
    } else {
        // Generic case - the from_types are specific types that should convert
        // to any instantiation of the generic struct
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        let expanded = quote! {
            #(
                impl<#generic_params> itemize_2::IntoItems<#ident #ty_generics> for #from_types
                #where_clause
                where
                    #ident #ty_generics: From<#from_types>,
                {
                    type IntoIter = ::std::iter::Once<#ident #ty_generics>;
                    fn into_items(self) -> Self::IntoIter {
                        ::std::iter::once(#ident::from(self))
                    }
                }
            )*
        };
        TokenStream::from(expanded)
    }
}
