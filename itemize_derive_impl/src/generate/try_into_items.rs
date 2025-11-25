use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    context::{CollectionType, Context},
    util::extract_lifetimes,
};

pub fn generate(ctx: &Context<'_>) -> TokenStream {
    let mut configs = vec![];

    for collection_type in &ctx.attributes.collections {
        configs.push(TryIntoItemsConfig::collection(ctx, collection_type).generate(ctx))
    }

    for type_ in &ctx.attributes.types {
        configs.push(TryIntoItemsConfig::type_(ctx, type_).generate(ctx))
    }

    if let Some(tuples) = ctx.attributes.tuples {
        for len in 1..=tuples {
            configs.push(TryIntoItemsConfig::tuple(ctx, len).generate(ctx))
        }
    }

    quote! { #(#configs)* }
}

pub struct TryIntoItemsConfig {
    target_ty: TokenStream,
    into_iter: TokenStream,
    into_impl: TokenStream,
    generics: TokenStream,
    conv_from: TokenStream,
}

impl TryIntoItemsConfig {
    fn collection(ctx: &Context<'_>, collection_type: &CollectionType) -> Self {
        let item_ty = format_ident!("__Item");
        let for_type = &ctx.for_type;
        let for_generics = &ctx.generics.params;

        let map_item_fn = quote! {
            fn map_item<Target, Item, E>(item: Item) -> Result<Target, E>
            where
                Target: ::std::convert::TryFrom<Item>,
                Target::Error: Into<E>,
            {
                Target::try_from(item).map_err(Into::into)
            }
        };

        let impl_body = match collection_type {
            CollectionType::Vec => {
                quote! { #map_item_fn self.into_iter().map(map_item::<#for_type, #item_ty, E>) }
            }
            CollectionType::Slice => {
                quote! { #map_item_fn self.iter().map(map_item::<#for_type, &'a #item_ty, E>) }
            }
            CollectionType::Array => {
                quote! { #map_item_fn self.into_iter().map(map_item::<#for_type, #item_ty, E>) }
            }
        };

        match collection_type {
            CollectionType::Vec => Self {
                into_iter: quote! { ::std::iter::Map<::std::vec::IntoIter<#item_ty>, fn(#item_ty) -> Result<#for_type, E>> },
                into_impl: impl_body,
                target_ty: quote! { Vec<#item_ty> },
                generics: quote! { <#item_ty, #for_generics, E> },
                conv_from: quote! { #for_type: ::std::convert::TryFrom<#item_ty>, <#for_type as ::std::convert::TryFrom<#item_ty>>::Error: Into<E> },
            },
            CollectionType::Slice => Self {
                into_iter: quote! { ::std::iter::Map<::std::slice::Iter<'a, #item_ty>, fn(&'a #item_ty) -> Result<#for_type, E>> },
                into_impl: impl_body,
                target_ty: quote! { &'a [#item_ty] },
                generics: quote! { <'a, #item_ty, #for_generics, E> },
                conv_from: quote! { #for_type: ::std::convert::TryFrom<&'a #item_ty>, <#for_type as ::std::convert::TryFrom<&'a #item_ty>>::Error: Into<E> },
            },
            CollectionType::Array => Self {
                into_iter: quote! { ::std::iter::Map<::std::array::IntoIter<#item_ty, N>, fn(#item_ty) -> Result<#for_type, E>> },
                into_impl: impl_body,
                target_ty: quote! { [#item_ty; N] },
                generics: quote! { <#item_ty, const N: usize, #for_generics, E> },
                conv_from: quote! { #for_type: ::std::convert::TryFrom<#item_ty>, <#for_type as ::std::convert::TryFrom<#item_ty>>::Error: Into<E> },
            },
        }
    }

    fn type_(ctx: &Context<'_>, type_: &syn::Type) -> Self {
        let for_type: &TokenStream = &ctx.for_type;

        let mut lifetimes = extract_lifetimes(type_);
        lifetimes.extend(ctx.generics.params.iter().map(|param| quote! { #param }));

        Self {
            into_iter: quote! { ::std::iter::Once<Result<#for_type, E>> },
            into_impl: quote! { ::std::iter::once(<#for_type as ::std::convert::TryFrom<#type_>>::try_from(self).map_err(Into::into)) },
            target_ty: quote! { #type_ },
            generics: quote! { <#(#lifetimes),*, E> },
            conv_from: quote! { #for_type: ::std::convert::TryFrom<#type_>, <#for_type as ::std::convert::TryFrom<#type_>>::Error: Into<E> },
        }
    }

    fn tuple(ctx: &Context<'_>, len: usize) -> Self {
        let for_type: &TokenStream = &ctx.for_type;

        let target = (0..len)
            .map(|i| format_ident!("_Item{}", i))
            .collect::<Vec<_>>();

        let mut generics = ctx
            .generics
            .params
            .iter()
            .map(|param| quote! { #param })
            .collect::<Vec<_>>();
        generics.extend(target.iter().map(|target| quote! { #target }));

        generics.push(quote! { E });

        let conv_from = target
            .iter()
            .map(|target| quote! { #for_type: ::std::convert::TryFrom<#target>, <#for_type as ::std::convert::TryFrom<#target>>::Error: Into<E> });

        let names = (0..len)
            .map(|i| format_ident!("a{}", i))
            .fold(quote! {}, |acc, x| {
                if len == 1 {
                    return quote! { #x, };
                }
                match acc.is_empty() {
                    true => quote! { #x },
                    false => quote! { #acc, #x },
                }
            });

        let block = (0..len)
            .map(|i| format_ident!("a{}", i))
            .map(|name| quote! { <#for_type>::try_from(#name).map_err(Into::into) })
            .fold(quote! {}, |acc, x| match acc.is_empty() {
                true => x,
                false => quote! { #acc, #x },
            });

        Self {
            into_iter: quote! { ::std::array::IntoIter<Result<#for_type, E>, #len> },
            into_impl: quote! {
                let (#names) = self;
                [#block].into_iter()
            },
            target_ty: quote! { (#(#target,)*) },
            generics: quote! { <#(#generics),*> },
            conv_from: quote! { #(#conv_from,)* },
        }
    }

    fn generate(self, ctx: &Context<'_>) -> TokenStream {
        let into_iter = self.into_iter;
        let into_impl = self.into_impl;
        let generics = self.generics;
        let target_ty = self.target_ty;
        let conv_from = self.conv_from;

        let predicates = ctx.where_predicates.iter().flatten();
        let for_type = &ctx.for_type;

        quote! {
            impl #generics itemize::TryIntoItems<#for_type, E> for #target_ty
            where
                #(#predicates,)*
                #conv_from
            {
                type IntoIter = #into_iter;
                fn try_into_items(self) -> Self::IntoIter {
                    #into_impl
                }
            }
        }
    }
}
