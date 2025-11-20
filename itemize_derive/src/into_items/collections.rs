use proc_macro2::TokenStream;
use quote::quote;

use super::context::Context;

pub fn from_collections(context: &Context) -> TokenStream {
    let mut impls = vec![];

    for collection_tokens in &context.attributes.collections {
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
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;

    // Check if the struct is generic
    if context.generics.params.is_empty() {
        // Non-generic case - use the simple type
        quote! {
            impl<T> itemize_2::IntoItems<#ident> for Vec<T>
            where
                #ident: From<T>,
            {
                type IntoIter = std::iter::Map<std::vec::IntoIter<T>, fn(T) -> #ident>;
                fn into_items(self) -> Self::IntoIter {
                    self.into_iter().map(#ident::from as fn(T) -> #ident)
                }
            }
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        quote! {
            impl<__CollectionItem, #generic_params> itemize_2::IntoItems<#ident #ty_generics> for Vec<__CollectionItem>
            #where_clause
            where
                #ident #ty_generics: From<__CollectionItem>,
            {
                type IntoIter = std::iter::Map<std::vec::IntoIter<__CollectionItem>, fn(__CollectionItem) -> #ident #ty_generics>;
                fn into_items(self) -> Self::IntoIter {
                    self.into_iter().map(#ident::from as fn(__CollectionItem) -> #ident #ty_generics)
                }
            }
        }
    }
}

fn generate_slice_impl(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;

    // Check if the struct is generic
    if context.generics.params.is_empty() {
        // Non-generic case - use the simple type
        quote! {
            impl<'a, T> itemize_2::IntoItems<#ident> for &'a [T]
            where
                #ident: From<&'a T>,
            {
                type IntoIter = std::iter::Map<std::slice::Iter<'a, T>, fn(&'a T) -> #ident>;
                fn into_items(self) -> Self::IntoIter {
                    self.iter().map(#ident::from as fn(&'a T) -> #ident)
                }
            }
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        quote! {
            impl<'a, __CollectionItem, #generic_params> itemize_2::IntoItems<#ident #ty_generics> for &'a [__CollectionItem]
            #where_clause
            where
                #ident #ty_generics: From<&'a __CollectionItem>,
            {
                type IntoIter = std::iter::Map<std::slice::Iter<'a, __CollectionItem>, fn(&'a __CollectionItem) -> #ident #ty_generics>;
                fn into_items(self) -> Self::IntoIter {
                    self.iter().map(#ident::from as fn(&'a __CollectionItem) -> #ident #ty_generics)
                }
            }
        }
    }
}

fn generate_array_impl(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;

    // Check if the struct is generic
    if context.generics.params.is_empty() {
        // Non-generic case - use the simple type
        quote! {
            impl<T, const N: usize> itemize_2::IntoItems<#ident> for [T; N]
            where
                #ident: From<T>,
            {
                type IntoIter = std::iter::Map<std::array::IntoIter<T, N>, fn(T) -> #ident>;
                fn into_items(self) -> Self::IntoIter {
                    IntoIterator::into_iter(self).map(#ident::from as fn(T) -> #ident)
                }
            }
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        quote! {
            impl<__CollectionItem, const N: usize, #generic_params> itemize_2::IntoItems<#ident #ty_generics> for [__CollectionItem; N]
            #where_clause
            where
                #ident #ty_generics: From<__CollectionItem>,
            {
                type IntoIter = std::iter::Map<std::array::IntoIter<__CollectionItem, N>, fn(__CollectionItem) -> #ident #ty_generics>;
                fn into_items(self) -> Self::IntoIter {
                    IntoIterator::into_iter(self).map(#ident::from as fn(__CollectionItem) -> #ident #ty_generics)
                }
            }
        }
    }
}
