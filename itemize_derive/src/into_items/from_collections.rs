use proc_macro2::TokenStream;
use quote::quote;

use super::context::Context;

pub fn from_collections(context: &Context) -> TokenStream {
    let trait_ident = &context.trait_ident;
    let ident = &context.ident;

    let mut impls = vec![];

    for collection_tokens in &context.attributes.from_collections {
        let impl_tokens = generate_collection_impl(collection_tokens, trait_ident, ident);
        impls.push(impl_tokens);
    }

    quote! {
        #(#impls)*
    }
}

fn generate_collection_impl(
    collection_tokens: &TokenStream,
    trait_ident: &syn::Ident,
    ident: &syn::Ident,
) -> TokenStream {
    // Convert TokenStream to string for pattern matching
    let collection_str = collection_tokens.to_string();

    // Handle different collection types
    if collection_str.starts_with("Vec") {
        generate_vec_impl(trait_ident, ident)
    } else if collection_str.starts_with("& [")
        || collection_str.starts_with("&[")
        || collection_str.starts_with("& ") && collection_str.contains("[")
    {
        generate_slice_impl(trait_ident, ident)
    } else if collection_str.contains("[") && collection_str.contains("; N]") {
        generate_array_impl(trait_ident, ident)
    } else {
        // Default case - should not happen with the expected inputs
        quote! {}
    }
}

fn generate_vec_impl(trait_ident: &syn::Ident, ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<T> #trait_ident for Vec<T>
        where
            #ident: From<T>,
        {
            type IntoIter = std::vec::IntoIter<#ident>;
            fn into_items(self) -> Self::IntoIter {
                self.into_iter()
                    .map(#ident::from)
                    .collect::<Vec<_>>()
                    .into_iter()
            }
        }
    }
}

fn generate_slice_impl(trait_ident: &syn::Ident, ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<'a, T> #trait_ident for &'a [T]
        where
            #ident: From<&'a T>,
        {
            type IntoIter = std::iter::Map<std::slice::Iter<'a, T>, fn(&'a T) -> #ident>;
            fn into_items(self) -> Self::IntoIter {
                self.iter().map(#ident::from as fn(&'a T) -> #ident)
            }
        }
    }
}

fn generate_array_impl(trait_ident: &syn::Ident, ident: &syn::Ident) -> TokenStream {
    quote! {
        impl<T, const N: usize> #trait_ident for [T; N]
        where
            #ident: From<T>,
        {
            type IntoIter = std::iter::Map<std::array::IntoIter<T, N>, fn(T) -> #ident>;
            fn into_items(self) -> Self::IntoIter {
                IntoIterator::into_iter(self).map(#ident::from as fn(T) -> #ident)
            }
        }
    }
}
