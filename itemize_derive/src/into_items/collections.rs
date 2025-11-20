use proc_macro2::TokenStream;
use quote::quote;

use super::context::Context;

pub fn from_collections(context: &Context) -> TokenStream {
    let mut impls = vec![];

    for collection_tokens in &context.attributes.from_collections {
        let impl_tokens = generate_collection_impl(&context, collection_tokens);
        impls.push(impl_tokens);
    }

    quote! {
        #(#impls)*
    }
}

fn generate_collection_impl(context: &Context, collection_tokens: &TokenStream) -> TokenStream {
    // Convert TokenStream to string for pattern matching
    let collection_str = collection_tokens.to_string();

    // Handle different collection types
    if collection_str.starts_with("Vec") {
        generate_vec_impl(&context)
    } else if collection_str.starts_with("& [")
        || collection_str.starts_with("&[")
        || collection_str.starts_with("& ") && collection_str.contains("[")
    {
        generate_slice_impl(&context)
    } else if collection_str.contains("[") && collection_str.contains("; N]") {
        generate_array_impl(&context)
    } else {
        // Default case - should not happen with the expected inputs
        quote! {}
    }
}

fn generate_vec_impl(context: &Context) -> TokenStream {
    let impl_generics = &context.impl_generics;
    let ty_generics = &context.ty_generics;
    let ident = &context.ident;

    quote! {
        impl<T> itemize_2::IntoItems<#ident #ty_generics> for Vec<T>
        where
            #ident #ty_generics: From<T>,
        {
            type IntoIter = std::iter::Map<std::vec::IntoIter<T>, fn(T) -> #ident #ty_generics>;
            fn into_items(self) -> Self::IntoIter {
                self.into_iter().map(#ident::from as fn(T) -> #ident #ty_generics)
            }
        }
    }
}

fn generate_slice_impl(context: &Context) -> TokenStream {
    let impl_generics = &context.impl_generics;
    let ty_generics = &context.ty_generics;
    let ident = &context.ident;

    quote! {
        impl<'a, T> itemize_2::IntoItems<#ident #ty_generics> for &'a [T]
        where
            #ident #ty_generics: From<&'a T>,
        {
            type IntoIter = std::iter::Map<std::slice::Iter<'a, T>, fn(&'a T) -> #ident #ty_generics>;
            fn into_items(self) -> Self::IntoIter {
                self.iter().map(#ident::from as fn(&'a T) -> #ident #ty_generics)
            }
        }
    }
}

fn generate_array_impl(context: &Context) -> TokenStream {
    let impl_generics = &context.impl_generics;
    let ty_generics = &context.ty_generics;
    let ident = &context.ident;

    quote! {
        impl<T, const N: usize> itemize_2::IntoItems<#ident #ty_generics> for [T; N]
        where
            #ident #ty_generics: From<T>,
        {
            type IntoIter = std::iter::Map<std::array::IntoIter<T, N>, fn(T) -> #ident #ty_generics>;
            fn into_items(self) -> Self::IntoIter {
                IntoIterator::into_iter(self).map(#ident::from as fn(T) -> #ident #ty_generics)
            }
        }
    }
}
