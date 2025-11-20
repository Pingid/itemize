use proc_macro2::TokenStream;
use quote::quote;

use crate::context::{CollectionType, Context};
use crate::util::{combine_where_clause, impl_generics_tokens, map_row_fn_tokens};

pub fn from_collections(context: &Context) -> TokenStream {
    let mut impls = vec![];

    for collection_tokens in &context.attributes.collections {
        match collection_tokens {
            CollectionType::Vec => impls.push(generate_vec_impl(&context)),
            CollectionType::Slice => impls.push(generate_slice_impl(&context)),
            CollectionType::Array => impls.push(generate_array_impl(&context)),
        }
    }

    quote! {
        #(#impls)*
    }
}

fn generate_vec_impl(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };

    build_collection_impl(
        context,
        &target,
        CollectionImplConfig {
            extra_impl_generics: vec![quote! { __RowItem }],
            collection_ty: quote! { Vec<__RowItem> },
            rows_type: quote! {
                ::std::iter::Map<::std::vec::IntoIter<__RowItem>, fn(__RowItem) -> Self::RowIter>
            },
            iter_expr: quote! { self.into_iter() },
            map_item_ty: quote! { __RowItem },
            items_source_ty: quote! { __RowItem },
        },
    )
}

fn generate_slice_impl(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };

    build_collection_impl(
        context,
        &target,
        CollectionImplConfig {
            extra_impl_generics: vec![quote! { 'a }, quote! { __RowItem }],
            collection_ty: quote! { &'a [__RowItem] },
            rows_type: quote! {
                ::std::iter::Map<::std::slice::Iter<'a, __RowItem>, fn(&'a __RowItem) -> Self::RowIter>
            },
            iter_expr: quote! { self.iter() },
            map_item_ty: quote! { &__RowItem },
            items_source_ty: quote! { &'a __RowItem },
        },
    )
}

fn generate_array_impl(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };

    build_collection_impl(
        context,
        &target,
        CollectionImplConfig {
            extra_impl_generics: vec![quote! { __RowItem }, quote! { const N: usize }],
            collection_ty: quote! { [__RowItem; N] },
            rows_type: quote! {
                ::std::iter::Map<::std::array::IntoIter<__RowItem, N>, fn(__RowItem) -> Self::RowIter>
            },
            iter_expr: quote! { IntoIterator::into_iter(self) },
            map_item_ty: quote! { __RowItem },
            items_source_ty: quote! { __RowItem },
        },
    )
}

fn build_collection_impl(
    context: &Context,
    target: &TokenStream,
    config: CollectionImplConfig,
) -> TokenStream {
    let CollectionImplConfig {
        extra_impl_generics,
        collection_ty,
        rows_type,
        iter_expr,
        map_item_ty,
        items_source_ty,
    } = config;

    let where_clause = combine_where_clause(
        context,
        ::std::iter::once(quote! { #items_source_ty: itemize::IntoItems<#target> }),
    );
    let map_row_fn = map_row_fn_tokens();
    let impl_generics = impl_generics_tokens(context, &extra_impl_generics);

    quote! {
        impl #impl_generics itemize::IntoRows<#target> for #collection_ty
        #where_clause
        {
            type RowIter = <#items_source_ty as itemize::IntoItems<#target>>::IntoIter;
            type Rows = #rows_type;
            fn into_rows(self) -> Self::Rows {
                #map_row_fn
                #iter_expr.map(map_row::<#target, #map_item_ty>)
            }
        }
    }
}

struct CollectionImplConfig {
    extra_impl_generics: Vec<TokenStream>,
    collection_ty: TokenStream,
    rows_type: TokenStream,
    iter_expr: TokenStream,
    map_item_ty: TokenStream,
    items_source_ty: TokenStream,
}
