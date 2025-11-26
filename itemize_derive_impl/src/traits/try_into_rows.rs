use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    context::{CollectionType, Context},
    util::{const_ident, item_ident, tuple_rows_associated, tuple_rows_impl, tuple_type_ident},
};

pub(crate) fn generate(ctx: &Context<'_>) -> TokenStream {
    let mut configs = vec![];

    for collection_type in &ctx.attributes.collections {
        configs.push(Config::from_collection(ctx, collection_type).generate(ctx))
    }

    if let Some(tuples) = ctx.attributes.tuples {
        for len in tuples.iter() {
            configs.push(Config::from_tuple(ctx, len).generate(ctx))
        }
    }

    quote! { #(#configs)* }
}

struct Config {
    concrete: TokenStream,
    associated_iter: TokenStream,
    associated_rows: TokenStream,
    body: TokenStream,
    generics: TokenStream,
    constraints: TokenStream,
    error_ty: TokenStream,
}

impl Config {
    fn from_collection(ctx: &Context<'_>, collection_type: &CollectionType) -> Self {
        let item_ty = item_ident();
        let const_ty = const_ident();
        let for_type = &ctx.concrete;

        match collection_type {
            CollectionType::Vec => {
                let generics = ctx
                    .error_generics()
                    .with_types([&item_ty])
                    .to_token_stream();
                let error_ty = ctx.error_ty();
                let associated_iter =
                    quote! { <#item_ty as itemize::TryIntoItems<#for_type, #error_ty>>::IntoIter };
                let map_fn = quote! { fn(#item_ty) -> #associated_iter };
                Self {
                    concrete: quote! { Vec<#item_ty> },
                    associated_iter,
                    associated_rows: quote! { ::std::iter::Map<::std::vec::IntoIter<#item_ty>, #map_fn> },
                    body: quote! { self.into_iter().map(<#item_ty as itemize::TryIntoItems<#for_type, #error_ty>>::try_into_items) },
                    generics,
                    constraints: quote! { #item_ty: itemize::TryIntoItems<#for_type, #error_ty> },
                    error_ty,
                }
            }
            CollectionType::Slice => {
                let generics = ctx
                    .error_generics()
                    .with_types([&item_ty])
                    .with_lifetimes([quote! { 'a }])
                    .to_token_stream();
                let error_ty = ctx.error_ty();
                let associated_iter = quote! { <&'a #item_ty as itemize::TryIntoItems<#for_type, #error_ty>>::IntoIter };
                let map_fn = quote! { fn(&'a #item_ty) -> #associated_iter };
                Self {
                    concrete: quote! { &'a [#item_ty] },
                    associated_iter,
                    associated_rows: quote! { ::std::iter::Map<::std::slice::Iter<'a, #item_ty>, #map_fn> },
                    body: quote! { self.iter().map(<&'a #item_ty as itemize::TryIntoItems<#for_type, #error_ty>>::try_into_items) },
                    generics,
                    constraints: quote! { &'a #item_ty: itemize::TryIntoItems<#for_type, #error_ty> },
                    error_ty,
                }
            }
            CollectionType::Array => {
                let generics = ctx
                    .error_generics()
                    .with_types([&item_ty])
                    .with_consts([quote! { const #const_ty: usize }])
                    .to_token_stream();
                let error_ty = ctx.error_ty();
                let associated_iter =
                    quote! { <#item_ty as itemize::TryIntoItems<#for_type, #error_ty>>::IntoIter };
                let map_fn = quote! { fn(#item_ty) -> #associated_iter };
                Self {
                    concrete: quote! { [#item_ty; #const_ty] },
                    associated_iter,
                    associated_rows: quote! { ::std::iter::Map<::std::array::IntoIter<#item_ty, #const_ty>, #map_fn> },
                    body: quote! { self.into_iter().map(<#item_ty as itemize::TryIntoItems<#for_type, #error_ty>>::try_into_items) },
                    generics,
                    constraints: quote! { #item_ty: itemize::TryIntoItems<#for_type, #error_ty> },
                    error_ty,
                }
            }
        }
    }

    fn from_tuple(ctx: &Context<'_>, len: usize) -> Self {
        let for_type: &TokenStream = &ctx.concrete;

        let target = (0..len).map(tuple_type_ident).collect::<Vec<_>>();

        let generics = ctx.error_generics().with_types(&target).to_token_stream();

        let error_ty = ctx.error_ty();
        let constraints = target
            .iter()
            .map(|target| quote! { #target: itemize::TryIntoItems<#for_type, #error_ty> });

        let iter_types: Vec<TokenStream> = target
            .iter()
            .map(|t| quote! { <#t as itemize::TryIntoItems<#for_type, #error_ty>>::IntoIter })
            .collect();

        let body = tuple_rows_impl(len, |name| quote! { #name.try_into_items() });

        Self {
            concrete: quote! { (#(#target,)*) },
            associated_iter: tuple_rows_associated(len, &iter_types, for_type),
            associated_rows: quote! { ::std::array::IntoIter<Self::RowIter, #len> },
            body,
            generics,
            constraints: quote! { #(#constraints,)* },
            error_ty,
        }
    }

    fn generate(self, ctx: &Context<'_>) -> TokenStream {
        let associated_iter = self.associated_iter;
        let associated_rows = self.associated_rows;
        let body = self.body;
        let generics = self.generics;
        let concrete = self.concrete;
        let constraints = self.constraints;
        let error_ty = self.error_ty;

        let predicates = ctx.where_predicates.iter().flatten();
        let for_type = &ctx.concrete;

        quote! {
            impl #generics itemize::TryIntoRows<#for_type, #error_ty> for #concrete
            where
                #(#predicates,)*
                #constraints
            {
                type RowIter = #associated_iter;
                type Rows = #associated_rows;
                #[inline]
                fn try_into_rows(self) -> Self::Rows {
                    #body
                }
            }
        }
    }
}
