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
        // Non-generic case
        quote! {
            impl<__RowItem> itemize_2::IntoRows<#ident> for Vec<__RowItem>
            where
                __RowItem: itemize_2::IntoItems<#ident>,
            {
                type RowIter = <__RowItem as itemize_2::IntoItems<#ident>>::IntoIter;
                type Rows = std::iter::Map<std::vec::IntoIter<__RowItem>, fn(__RowItem) -> Self::RowIter>;
                fn into_rows(self) -> Self::Rows {
                    fn map_row<Target, Item>(item: Item) -> <Item as itemize_2::IntoItems<Target>>::IntoIter
                    where
                        Item: itemize_2::IntoItems<Target>,
                    {
                        item.into_items()
                    }
                    self.into_iter().map(map_row::<#ident, __RowItem>)
                }
            }
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        quote! {
            impl<__RowItem, #generic_params> itemize_2::IntoRows<#ident #ty_generics> for Vec<__RowItem>
            #where_clause
            where
                __RowItem: itemize_2::IntoItems<#ident #ty_generics>,
            {
                type RowIter = <__RowItem as itemize_2::IntoItems<#ident #ty_generics>>::IntoIter;
                type Rows = std::iter::Map<std::vec::IntoIter<__RowItem>, fn(__RowItem) -> Self::RowIter>;
                fn into_rows(self) -> Self::Rows {
                    fn map_row<Target, Item>(item: Item) -> <Item as itemize_2::IntoItems<Target>>::IntoIter
                    where
                        Item: itemize_2::IntoItems<Target>,
                    {
                        item.into_items()
                    }
                    self.into_iter().map(map_row::<#ident #ty_generics, __RowItem>)
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
        // Non-generic case
        quote! {
            impl<'a, __RowItem> itemize_2::IntoRows<#ident> for &'a [__RowItem]
            where
                &'a __RowItem: itemize_2::IntoItems<#ident>,
            {
                type RowIter = <&'a __RowItem as itemize_2::IntoItems<#ident>>::IntoIter;
                type Rows = std::iter::Map<std::slice::Iter<'a, __RowItem>, fn(&'a __RowItem) -> Self::RowIter>;
                fn into_rows(self) -> Self::Rows {
                    fn map_row<Target, Item>(item: Item) -> <Item as itemize_2::IntoItems<Target>>::IntoIter
                    where
                        Item: itemize_2::IntoItems<Target>,
                    {
                        item.into_items()
                    }
                    self.iter().map(map_row::<#ident, &__RowItem>)
                }
            }
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        quote! {
            impl<'a, __RowItem, #generic_params> itemize_2::IntoRows<#ident #ty_generics> for &'a [__RowItem]
            #where_clause
            where
                &'a __RowItem: itemize_2::IntoItems<#ident #ty_generics>,
            {
                type RowIter = <&'a __RowItem as itemize_2::IntoItems<#ident #ty_generics>>::IntoIter;
                type Rows = std::iter::Map<std::slice::Iter<'a, __RowItem>, fn(&'a __RowItem) -> Self::RowIter>;
                fn into_rows(self) -> Self::Rows {
                    fn map_row<Target, Item>(item: Item) -> <Item as itemize_2::IntoItems<Target>>::IntoIter
                    where
                        Item: itemize_2::IntoItems<Target>,
                    {
                        item.into_items()
                    }
                    self.iter().map(map_row::<#ident #ty_generics, &__RowItem>)
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
        // Non-generic case
        quote! {
            impl<__RowItem, const N: usize> itemize_2::IntoRows<#ident> for [__RowItem; N]
            where
                __RowItem: itemize_2::IntoItems<#ident>,
            {
                type RowIter = <__RowItem as itemize_2::IntoItems<#ident>>::IntoIter;
                type Rows = std::iter::Map<std::array::IntoIter<__RowItem, N>, fn(__RowItem) -> Self::RowIter>;
                fn into_rows(self) -> Self::Rows {
                    fn map_row<Target, Item>(item: Item) -> <Item as itemize_2::IntoItems<Target>>::IntoIter
                    where
                        Item: itemize_2::IntoItems<Target>,
                    {
                        item.into_items()
                    }
                    IntoIterator::into_iter(self).map(map_row::<#ident, __RowItem>)
                }
            }
        }
    } else {
        // Generic case - use the generic params directly
        let generic_params = &context.generics.params;
        let where_clause = &context.where_clause;

        quote! {
            impl<__RowItem, const N: usize, #generic_params> itemize_2::IntoRows<#ident #ty_generics> for [__RowItem; N]
            #where_clause
            where
                __RowItem: itemize_2::IntoItems<#ident #ty_generics>,
            {
                type RowIter = <__RowItem as itemize_2::IntoItems<#ident #ty_generics>>::IntoIter;
                type Rows = std::iter::Map<std::array::IntoIter<__RowItem, N>, fn(__RowItem) -> Self::RowIter>;
                fn into_rows(self) -> Self::Rows {
                    fn map_row<Target, Item>(item: Item) -> <Item as itemize_2::IntoItems<Target>>::IntoIter
                    where
                        Item: itemize_2::IntoItems<Target>,
                    {
                        item.into_items()
                    }
                    IntoIterator::into_iter(self).map(map_row::<#ident #ty_generics, __RowItem>)
                }
            }
        }
    }
}
