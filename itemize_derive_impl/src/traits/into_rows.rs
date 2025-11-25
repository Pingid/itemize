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
}

impl Config {
    fn from_collection(ctx: &Context<'_>, collection_type: &CollectionType) -> Self {
        let item_ty = item_ident();
        let const_ty = const_ident();
        let for_type = &ctx.concrete;

        let generics = ctx.generics().with_types([&item_ty]);

        match collection_type {
            CollectionType::Vec => {
                let associated_iter =
                    quote! { <#item_ty as itemize::IntoItems<#for_type>>::IntoIter };
                let map_fn = quote! { fn(#item_ty) -> #associated_iter };
                Self {
                    concrete: quote! { Vec<#item_ty> },
                    associated_iter,
                    associated_rows: quote! { ::std::iter::Map<::std::vec::IntoIter<#item_ty>, #map_fn> },
                    body: quote! { self.into_iter().map(<#item_ty as itemize::IntoItems<#for_type>>::into_items) },
                    generics: generics.to_token_stream(),
                    constraints: quote! { #item_ty: itemize::IntoItems<#for_type> },
                }
            }
            CollectionType::Slice => {
                let associated_iter =
                    quote! { <&'a #item_ty as itemize::IntoItems<#for_type>>::IntoIter };
                let map_fn = quote! { fn(&'a #item_ty) -> #associated_iter };
                Self {
                    concrete: quote! { &'a [#item_ty] },
                    associated_iter,
                    associated_rows: quote! { ::std::iter::Map<::std::slice::Iter<'a, #item_ty>, #map_fn> },
                    body: quote! { self.iter().map(<&'a #item_ty as itemize::IntoItems<#for_type>>::into_items) },
                    generics: generics.with_lifetimes([quote! { 'a }]).to_token_stream(),
                    constraints: quote! { &'a #item_ty: itemize::IntoItems<#for_type> },
                }
            }
            CollectionType::Array => {
                let associated_iter =
                    quote! { <#item_ty as itemize::IntoItems<#for_type>>::IntoIter };
                let map_fn = quote! { fn(#item_ty) -> #associated_iter };
                Self {
                    concrete: quote! { [#item_ty; #const_ty] },
                    associated_iter,
                    associated_rows: quote! { ::std::iter::Map<::std::array::IntoIter<#item_ty, #const_ty>, #map_fn> },
                    body: quote! { self.into_iter().map(<#item_ty as itemize::IntoItems<#for_type>>::into_items) },
                    generics: generics
                        .with_consts([quote! { const #const_ty: usize }])
                        .to_token_stream(),
                    constraints: quote! { #item_ty: itemize::IntoItems<#for_type> },
                }
            }
        }
    }

    fn from_tuple(ctx: &Context<'_>, len: usize) -> Self {
        let for_type: &TokenStream = &ctx.concrete;

        let target = (0..len).map(tuple_type_ident).collect::<Vec<_>>();

        let generics = ctx.generics().with_types(&target).to_token_stream();

        let constraints = target
            .iter()
            .map(|target| quote! { #target: itemize::IntoItems<#for_type> });

        let iter_types: Vec<TokenStream> = target
            .iter()
            .map(|t| quote! { <#t as itemize::IntoItems<#for_type>>::IntoIter })
            .collect();

        let body = tuple_rows_impl(len, |name| quote! { #name.into_items() });

        Self {
            concrete: quote! { (#(#target,)*) },
            associated_iter: tuple_rows_associated(len, &iter_types, for_type),
            associated_rows: quote! { ::std::array::IntoIter<Self::RowIter, #len> },
            body,
            generics,
            constraints: quote! { #(#constraints,)* },
        }
    }

    fn generate(self, ctx: &Context<'_>) -> TokenStream {
        let row_iter = self.associated_iter;
        let rows = self.associated_rows;
        let into_impl = self.body;
        let generics = self.generics;
        let target_ty = self.concrete;
        let conv_from = self.constraints;

        let predicates = ctx.where_predicates.iter().flatten();
        let for_type = &ctx.concrete;

        quote! {
            impl #generics itemize::IntoRows<#for_type> for #target_ty
            where
                #(#predicates,)*
                #conv_from
            {
                type RowIter = #row_iter;
                type Rows = #rows;
                #[inline]
                fn into_rows(self) -> Self::Rows {
                    #into_impl
                }
            }
        }
    }
}
