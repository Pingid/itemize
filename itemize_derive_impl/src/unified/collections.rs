use proc_macro2::TokenStream;
use quote::quote;

use crate::context::{CollectionType, Context};
use crate::trait_config::{TraitConfig, TraitKind};
use crate::util::{
    build_ordered_impl_generics, combine_where_clause, map_item_fn_tokens, map_row_fn_tokens,
};

pub fn from_collections(context: &Context, config: &TraitConfig) -> TokenStream {
    let mut impls = Vec::new();

    for collection_type in &context.attributes.collections {
        let tokens = match collection_type {
            CollectionType::Vec => generate_vec_impl(context, config),
            CollectionType::Slice => generate_slice_impl(context, config),
            CollectionType::Array => generate_array_impl(context, config),
        };
        impls.push(tokens);
    }

    quote! { #(#impls)* }
}

fn generate_vec_impl(context: &Context, config: &TraitConfig) -> TokenStream {
    match config.kind {
        TraitKind::Items => build_items_collection_impl(
            context,
            config,
            ItemsCollectionConfig {
                extra_impl_generics: vec![quote! { __CollectionItem }],
                collection_ty: quote! { Vec<__CollectionItem> },
                iterator_ty: quote! { ::std::vec::IntoIter<__CollectionItem> },
                iter_expr: quote! { self.into_iter() },
                map_item_ty: quote! { __CollectionItem },
                items_source_ty: quote! { __CollectionItem },
            },
        ),
        TraitKind::Rows => {
            let ident = &context.ident;
            let ty_generics = &context.ty_generics;
            let target = quote! { #ident #ty_generics };

            build_rows_collection_impl(
                context,
                config,
                &target,
                RowsCollectionConfig {
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
    }
}

fn generate_slice_impl(context: &Context, config: &TraitConfig) -> TokenStream {
    match config.kind {
        TraitKind::Items => build_items_collection_impl(
            context,
            config,
            ItemsCollectionConfig {
                extra_impl_generics: vec![quote! { 'a }, quote! { __CollectionItem }],
                collection_ty: quote! { &'a [__CollectionItem] },
                iterator_ty: quote! { ::std::slice::Iter<'a, __CollectionItem> },
                iter_expr: quote! { self.iter() },
                map_item_ty: quote! { &'a __CollectionItem },
                items_source_ty: quote! { &'a __CollectionItem },
            },
        ),
        TraitKind::Rows => {
            let ident = &context.ident;
            let ty_generics = &context.ty_generics;
            let target = quote! { #ident #ty_generics };

            build_rows_collection_impl(
                context,
                config,
                &target,
                RowsCollectionConfig {
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
    }
}

fn generate_array_impl(context: &Context, config: &TraitConfig) -> TokenStream {
    match config.kind {
        TraitKind::Items => build_items_collection_impl(
            context,
            config,
            ItemsCollectionConfig {
                extra_impl_generics: vec![quote! { __CollectionItem }, quote! { const N: usize }],
                collection_ty: quote! { [__CollectionItem; N] },
                iterator_ty: quote! { ::std::array::IntoIter<__CollectionItem, N> },
                iter_expr: quote! { IntoIterator::into_iter(self) },
                map_item_ty: quote! { __CollectionItem },
                items_source_ty: quote! { __CollectionItem },
            },
        ),
        TraitKind::Rows => {
            let ident = &context.ident;
            let ty_generics = &context.ty_generics;
            let target = quote! { #ident #ty_generics };

            build_rows_collection_impl(
                context,
                config,
                &target,
                RowsCollectionConfig {
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
    }
}

fn build_items_collection_impl(
    context: &Context,
    config: &TraitConfig,
    impl_config: ItemsCollectionConfig,
) -> TokenStream {
    let ItemsCollectionConfig {
        extra_impl_generics,
        collection_ty,
        iterator_ty,
        iter_expr,
        map_item_ty,
        items_source_ty,
    } = impl_config;

    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };
    let trait_path = config.trait_path();
    let trait_generics = config.trait_generics(&target);
    let method_name = config.method_name();

    let impl_generics = build_ordered_impl_generics(context, config, &extra_impl_generics);
    let map_fn = map_item_fn_tokens();

    let where_clause = if config.is_try() {
        let error_type = config.error_type_tokens();
        combine_where_clause(
            context,
            [quote! {
                #target: ::std::convert::TryFrom<#map_item_ty>,
                <#target as ::std::convert::TryFrom<#map_item_ty>>::Error: Into<#error_type>
            }],
        )
    } else {
        combine_where_clause(
            context,
            [quote! { #target: ::std::convert::From<#items_source_ty> }],
        )
    };

    let (into_iter_type, map_expr) = if config.is_try() {
        let error_type = config.error_type_tokens();
        let into_iter_type = quote! {
            ::std::iter::Map<#iterator_ty, fn(#map_item_ty) -> Result<#target, #error_type>>
        };
        let map_expr = quote! {
            fn try_map_item<Target, Item, E>(item: Item) -> Result<Target, E>
            where
                Target: ::std::convert::TryFrom<Item>,
                <Target as ::std::convert::TryFrom<Item>>::Error: Into<E>,
            {
                Target::try_from(item).map_err(Into::into)
            }
            #iter_expr.map(try_map_item::<#target, #map_item_ty, #error_type>)
        };
        (into_iter_type, map_expr)
    } else {
        let into_iter_type = quote! {
            ::std::iter::Map<#iterator_ty, fn(#map_item_ty) -> #target>
        };
        let map_expr = quote! {
            #map_fn
            #iter_expr.map(map_item::<#target, #map_item_ty>)
        };
        (into_iter_type, map_expr)
    };

    quote! {
        impl #impl_generics #trait_path<#trait_generics> for #collection_ty
        #where_clause
        {
            type IntoIter = #into_iter_type;
            fn #method_name(self) -> Self::IntoIter {
                #map_expr
            }
        }
    }
}

fn build_rows_collection_impl(
    context: &Context,
    config: &TraitConfig,
    target: &TokenStream,
    impl_config: RowsCollectionConfig,
) -> TokenStream {
    let RowsCollectionConfig {
        extra_impl_generics,
        collection_ty,
        rows_type,
        iter_expr,
        map_item_ty,
        items_source_ty,
    } = impl_config;

    let trait_path = config.trait_path();
    let trait_generics = config.trait_generics(target);
    let method_name = config.method_name();

    let (trait_constraint, row_iter_type) = if config.is_try() {
        let error_type = config.error_type_tokens();
        (
            quote! { #items_source_ty: itemize::TryIntoItems<#target, #error_type> },
            quote! { <#items_source_ty as itemize::TryIntoItems<#target, #error_type>>::IntoIter },
        )
    } else {
        (
            quote! { #items_source_ty: itemize::IntoItems<#target> },
            quote! { <#items_source_ty as itemize::IntoItems<#target>>::IntoIter },
        )
    };

    let where_clause = combine_where_clause(context, [trait_constraint]);
    let map_row_fn = map_row_fn_tokens();
    let impl_generics = build_ordered_impl_generics(context, config, &extra_impl_generics);

    let map_expr = if config.is_try() {
        let error_type = config.error_type_tokens();
        quote! {
            fn try_map_row<Target, Item, E>(item: Item) -> <Item as itemize::TryIntoItems<Target, E>>::IntoIter
            where
                Item: itemize::TryIntoItems<Target, E>,
            {
                item.try_into_items()
            }
            #iter_expr.map(try_map_row::<#target, #map_item_ty, #error_type>)
        }
    } else {
        quote! {
            #map_row_fn
            #iter_expr.map(map_row::<#target, #map_item_ty>)
        }
    };

    quote! {
        impl #impl_generics #trait_path<#trait_generics> for #collection_ty
        #where_clause
        {
            type RowIter = #row_iter_type;
            type Rows = #rows_type;
            fn #method_name(self) -> Self::Rows {
                #map_expr
            }
        }
    }
}

struct ItemsCollectionConfig {
    extra_impl_generics: Vec<TokenStream>,
    collection_ty: TokenStream,
    iterator_ty: TokenStream,
    iter_expr: TokenStream,
    map_item_ty: TokenStream,
    items_source_ty: TokenStream,
}

struct RowsCollectionConfig {
    extra_impl_generics: Vec<TokenStream>,
    collection_ty: TokenStream,
    rows_type: TokenStream,
    iter_expr: TokenStream,
    map_item_ty: TokenStream,
    items_source_ty: TokenStream,
}
