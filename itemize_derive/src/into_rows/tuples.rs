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
        for n in 2..=max {
            // Generate tuple type parameters
            let type_params: Vec<_> = (0..n).map(|i| format_ident!("__RowItem{}", i)).collect();
            let indices: Vec<_> = (0..n).map(|i| syn::Index::from(i)).collect();
            
            // For IntoRows, each tuple element becomes a separate row
            // We need to create an iterator of iterators
            let array_elements: Vec<_> = indices
                .iter()
                .map(|idx| quote! { self.#idx.into_items() })
                .collect();

            // Create the Rows iterator type - it's an array of row iterators
            let rows_type = quote! {
                std::array::IntoIter<
                    Box<dyn std::iter::Iterator<Item = #ident>>,
                    #n
                >
            };

            expanded.extend(quote! {
                impl<#(#type_params),*> itemize_2::IntoRows<#ident> for (#(#type_params,)*)
                where
                    #(#type_params: itemize_2::IntoItems<#ident>,)*
                    #(<#type_params as itemize_2::IntoItems<#ident>>::IntoIter: 'static,)*
                {
                    type RowIter = Box<dyn std::iter::Iterator<Item = #ident>>;
                    type Rows = #rows_type;

                    fn into_rows(self) -> Self::Rows {
                        [
                            #(Box::new(#array_elements) as Box<dyn std::iter::Iterator<Item = #ident>>,)*
                        ].into_iter()
                    }
                }
            });
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        for n in 2..=max {
            // For tuples, each element must implement IntoItems and becomes a separate row
            let type_params: Vec<_> = (0..n).map(|i| format_ident!("__RowItem{}", i)).collect();
            let indices: Vec<_> = (0..n).map(|i| syn::Index::from(i)).collect();
            
            // Each tuple element becomes a row
            let array_elements: Vec<_> = indices
                .iter()
                .map(|idx| quote! { self.#idx.into_items() })
                .collect();

            // Create the Rows iterator type
            let rows_type = quote! {
                std::array::IntoIter<
                    Box<dyn std::iter::Iterator<Item = #ident #ty_generics>>,
                    #n
                >
            };

            expanded.extend(quote! {
                impl<#(#type_params,)* #generic_params> itemize_2::IntoRows<#ident #ty_generics> for (#(#type_params,)*)
                #where_clause
                where
                    #(#type_params: itemize_2::IntoItems<#ident #ty_generics>,)*
                    #(<#type_params as itemize_2::IntoItems<#ident #ty_generics>>::IntoIter: 'static,)*
                {
                    type RowIter = Box<dyn std::iter::Iterator<Item = #ident #ty_generics>>;
                    type Rows = #rows_type;

                    fn into_rows(self) -> Self::Rows {
                        [
                            #(Box::new(#array_elements) as Box<dyn std::iter::Iterator<Item = #ident #ty_generics>>,)*
                        ].into_iter()
                    }
                }
            });
        }
    }

    expanded
}