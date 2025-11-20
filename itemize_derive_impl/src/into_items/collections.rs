use proc_macro2::TokenStream;
use quote::quote;

use crate::context::{CollectionType, Context};
use crate::util::{combine_where_clause, impl_generics_tokens, map_item_fn_tokens};

pub fn from_collections(context: &Context) -> TokenStream {
    let mut impls = Vec::new();

    for collection_tokens in &context.attributes.collections {
        let tokens = match collection_tokens {
            CollectionType::Vec => generate_vec_impl(context),
            CollectionType::Slice => generate_slice_impl(context),
            CollectionType::Array => generate_array_impl(context),
        };
        impls.push(tokens);
    }

    quote! {
        #(#impls)*
    }
}

fn generate_vec_impl(context: &Context) -> TokenStream {
    build_collection_impl(
        context,
        CollectionImplConfig {
            extra_impl_generics: vec![quote! { __CollectionItem }],
            collection_ty: quote! { Vec<__CollectionItem> },
            iterator_ty: quote! { ::std::vec::IntoIter<__CollectionItem> },
            iter_expr: quote! { self.into_iter() },
            map_item_ty: quote! { __CollectionItem },
            items_source_ty: quote! { __CollectionItem },
        },
    )
}

fn generate_slice_impl(context: &Context) -> TokenStream {
    build_collection_impl(
        context,
        CollectionImplConfig {
            extra_impl_generics: vec![quote! { 'a }, quote! { __CollectionItem }],
            collection_ty: quote! { &'a [__CollectionItem] },
            iterator_ty: quote! { ::std::slice::Iter<'a, __CollectionItem> },
            iter_expr: quote! { self.iter() },
            map_item_ty: quote! { &'a __CollectionItem },
            items_source_ty: quote! { &'a __CollectionItem },
        },
    )
}

fn generate_array_impl(context: &Context) -> TokenStream {
    build_collection_impl(
        context,
        CollectionImplConfig {
            extra_impl_generics: vec![quote! { __CollectionItem }, quote! { const N: usize }],
            collection_ty: quote! { [__CollectionItem; N] },
            iterator_ty: quote! { ::std::array::IntoIter<__CollectionItem, N> },
            iter_expr: quote! { IntoIterator::into_iter(self) },
            map_item_ty: quote! { __CollectionItem },
            items_source_ty: quote! { __CollectionItem },
        },
    )
}

fn build_collection_impl(context: &Context, config: CollectionImplConfig) -> TokenStream {
    let CollectionImplConfig {
        extra_impl_generics,
        collection_ty,
        iterator_ty,
        iter_expr,
        map_item_ty,
        items_source_ty,
    } = config;

    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };
    let impl_generics = impl_generics_tokens(context, &extra_impl_generics);
    let map_fn = map_item_fn_tokens();
    let where_clause = combine_where_clause(
        context,
        ::std::iter::once(quote! { #target: ::std::convert::From<#items_source_ty> }),
    );

    quote! {
        impl #impl_generics itemize::IntoItems<#target> for #collection_ty
        #where_clause
        {
            type IntoIter = ::std::iter::Map<#iterator_ty, fn(#map_item_ty) -> #target>;
            fn into_items(self) -> Self::IntoIter {
                #map_fn
                #iter_expr.map(map_item::<#target, #map_item_ty>)
            }
        }
    }
}

struct CollectionImplConfig {
    extra_impl_generics: Vec<TokenStream>,
    collection_ty: TokenStream,
    iterator_ty: TokenStream,
    iter_expr: TokenStream,
    map_item_ty: TokenStream,
    items_source_ty: TokenStream,
}
