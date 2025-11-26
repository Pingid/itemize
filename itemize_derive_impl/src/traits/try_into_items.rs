use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    context::{CollectionType, Context},
    util::{const_ident, item_ident, tuple_items_impl, tuple_type_ident},
};

pub(crate) fn generate(ctx: &Context<'_>) -> TokenStream {
    let mut configs = vec![];

    for collection_type in &ctx.attributes.collections {
        configs.push(Config::from_collection(ctx, collection_type).generate(ctx))
    }

    for type_ in &ctx.attributes.types {
        configs.push(Config::from_type(ctx, type_).generate(ctx))
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
    associated: TokenStream,
    body: TokenStream,
    generics: TokenStream,
    constraints: TokenStream,
    error_ty: TokenStream,
}

impl Config {
    fn from_type(ctx: &Context<'_>, type_: &syn::Type) -> Self {
        let error_ty = ctx.error_ty();
        let item: &TokenStream = &ctx.concrete;
        let generics = ctx
            .error_generics()
            .with_lifetimes_from_type(type_)
            .to_token_stream();

        Self {
            associated: quote! { ::std::iter::Once<Result<#item, #error_ty>> },
            body: quote! { ::std::iter::once(<#item as ::std::convert::TryFrom<#type_>>::try_from(self).map_err(Into::into)) },
            concrete: quote! { #type_ },
            generics,
            constraints: quote! { #item: ::std::convert::TryFrom<#type_>, <#item as ::std::convert::TryFrom<#type_>>::Error: Into<#error_ty> },
            error_ty,
        }
    }

    fn from_collection(ctx: &Context<'_>, collection_type: &CollectionType) -> Self {
        let item_ty = item_ident();
        let const_ty = const_ident();
        let for_type = &ctx.concrete;

        let map_item = map_item_try_from();

        match collection_type {
            CollectionType::Vec => {
                let generics = ctx
                    .error_generics()
                    .with_types([&item_ty])
                    .to_token_stream();
                let error_ty = ctx.error_ty();
                Self {
                    associated: quote! { ::std::iter::Map<::std::vec::IntoIter<#item_ty>, fn(#item_ty) -> Result<#for_type, #error_ty>> },
                    body: quote! { #map_item self.into_iter().map(map_item::<#for_type, #item_ty, #error_ty>) },
                    concrete: quote! { Vec<#item_ty> },
                    generics,
                    constraints: quote! { #for_type: ::std::convert::TryFrom<#item_ty>, <#for_type as ::std::convert::TryFrom<#item_ty>>::Error: Into<#error_ty> },
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
                Self {
                    associated: quote! { ::std::iter::Map<::std::slice::Iter<'a, #item_ty>, fn(&'a #item_ty) -> Result<#for_type, #error_ty>> },
                    body: quote! { #map_item self.iter().map(map_item::<#for_type, &'a #item_ty, #error_ty>) },
                    concrete: quote! { &'a [#item_ty] },
                    generics,
                    constraints: quote! { #for_type: ::std::convert::TryFrom<&'a #item_ty>, <#for_type as ::std::convert::TryFrom<&'a #item_ty>>::Error: Into<#error_ty> },
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
                Self {
                    associated: quote! { ::std::iter::Map<::std::array::IntoIter<#item_ty, #const_ty>, fn(#item_ty) -> Result<#for_type, #error_ty>> },
                    body: quote! { #map_item self.into_iter().map(map_item::<#for_type, #item_ty, #error_ty>) },
                    concrete: quote! { [#item_ty; #const_ty] },
                    generics,
                    constraints: quote! { #for_type: ::std::convert::TryFrom<#item_ty>, <#for_type as ::std::convert::TryFrom<#item_ty>>::Error: Into<#error_ty> },
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
        let constraint = target
            .iter()
            .map(|target| quote! { #for_type: ::std::convert::TryFrom<#target>, <#for_type as ::std::convert::TryFrom<#target>>::Error: Into<#error_ty> });

        let body = tuple_items_impl(
            len,
            |name| quote! { <#for_type>::try_from(#name).map_err(Into::into) },
            |block| quote! { [#block].into_iter() },
        );

        Self {
            associated: quote! { ::std::array::IntoIter<Result<#for_type, #error_ty>, #len> },
            body,
            concrete: quote! { (#(#target,)*) },
            generics,
            constraints: quote! { #(#constraint,)* },
            error_ty,
        }
    }

    fn generate(self, ctx: &Context<'_>) -> TokenStream {
        let associated = self.associated;
        let body = self.body;
        let generics = self.generics;
        let concrete = self.concrete;
        let constraints = self.constraints;
        let error_ty = self.error_ty;

        let predicates = ctx.where_predicates.iter().flatten();
        let item = &ctx.concrete;

        quote! {
            impl #generics itemize::TryIntoItems<#item, #error_ty> for #concrete
            where
                #(#predicates,)*
                #constraints
            {
                type IntoIter = #associated;
                #[inline]
                fn try_into_items(self) -> Self::IntoIter {
                    #body
                }
            }
        }
    }
}

fn map_item_try_from() -> TokenStream {
    quote! {
        #[inline]
        fn map_item<Target, Item, E>(item: Item) -> Result<Target, E>
        where
            Target: ::std::convert::TryFrom<Item>,
            Target::Error: Into<E>,
        {
            Target::try_from(item).map_err(Into::into)
        }
    }
}
