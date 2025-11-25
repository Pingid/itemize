use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::context::{CollectionType, Context};

pub fn generate(ctx: &Context<'_>) -> TokenStream {
    let mut configs = vec![];

    for collection_type in &ctx.attributes.collections {
        configs.push(IntoRowsConfig::collection(ctx, collection_type).generate(ctx))
    }

    if let Some(tuples) = ctx.attributes.tuples {
        for len in 1..=tuples {
            configs.push(IntoRowsConfig::tuple(ctx, len).generate(ctx))
        }
    }

    quote! { #(#configs)* }
}

pub struct IntoRowsConfig {
    target_ty: TokenStream,
    row_iter: TokenStream,
    rows: TokenStream,
    into_impl: TokenStream,
    generics: TokenStream,
    conv_from: TokenStream,
}

impl IntoRowsConfig {
    fn tuple(ctx: &Context<'_>, len: usize) -> Self {
        let for_type: &TokenStream = &ctx.for_type;

        let target = (0..len)
            .map(|i| format_ident!("A{}", i))
            .collect::<Vec<_>>();

        let mut generics = ctx
            .generics
            .params
            .iter()
            .map(|param| quote! { #param })
            .collect::<Vec<_>>();
        generics.extend(target.iter().map(|target| quote! { #target }));

        let conv_from = target
            .iter()
            .map(|target| quote! { #target: itemize::IntoItems<#for_type> });

        let iter_types: Vec<TokenStream> = target.iter()
            .map(|t| quote! { <#t as itemize::IntoItems<#for_type>>::IntoIter })
            .collect();
        
        // Helper to build recursive Either type
        fn build_type(types: &[TokenStream]) -> TokenStream {
            if types.len() == 1 {
                types[0].clone()
            } else {
                let head = &types[0];
                let tail = build_type(&types[1..]);
                quote! { itemize::Either<#head, #tail> }
            }
        }
        
        let row_iter = if len == 0 {
            quote! { ::std::iter::Empty<#for_type> }
        } else {
            build_type(&iter_types)
        };

        let names = (0..len)
            .map(|i| format_ident!("a{}", i))
            .collect::<Vec<_>>();

        let destructure = if len == 1 {
             // For 1-tuple (a,), destructuring with (a,) works.
             let n = &names[0];
             quote! { let (#n,) = self; }
        } else {
             quote! { let (#(#names),*) = self; }
        };

        // Helper to build recursive Either value
        fn build_val(idx: usize, len: usize, expr: TokenStream) -> TokenStream {
            if len == 1 {
                expr
            } else if idx == 0 {
                quote! { itemize::Either::Left(#expr) }
            } else {
                let inner = build_val(idx - 1, len - 1, expr);
                quote! { itemize::Either::Right(#inner) }
            }
        }

        let exprs = names.iter().enumerate().map(|(i, name)| {
            let base = quote! { #name.into_items() };
            build_val(i, len, base)
        });

        Self {
            row_iter,
            rows: quote! { ::std::array::IntoIter<Self::RowIter, #len> },
            into_impl: quote! {
                #destructure
                [#(#exprs),*].into_iter()
            },
            target_ty: quote! { (#(#target,)*) },
            generics: quote! { <#(#generics),*> },
            conv_from: quote! { #(#conv_from,)* },
        }
    }

    fn collection(ctx: &Context<'_>, collection_type: &CollectionType) -> Self {
        let item_ty = format_ident!("__Item");
        let for_type = &ctx.for_type;
        let for_generics = &ctx.generics.params;

        // collection<T> implements IntoRows<Row>
        // Requires T: IntoItems<Row>
        
        let (into_iter, target_ty, generics, conv_from, row_iter, map_fn) = match collection_type {
            CollectionType::Vec => {
                let iter = quote! { ::std::vec::IntoIter<#item_ty> };
                let target = quote! { Vec<#item_ty> };
                let gens = quote! { <#item_ty, #for_generics> };
                let conv = quote! { #item_ty: itemize::IntoItems<#for_type> };
                let r_iter = quote! { <#item_ty as itemize::IntoItems<#for_type>>::IntoIter };
                let m_fn = quote! { fn(#item_ty) -> #r_iter };
                (iter, target, gens, conv, r_iter, m_fn)
            }
            CollectionType::Slice => {
                let iter = quote! { ::std::slice::Iter<'a, #item_ty> };
                let target = quote! { &'a [#item_ty] };
                let gens = quote! { <'a, #item_ty, #for_generics> };
                let conv = quote! { &'a #item_ty: itemize::IntoItems<#for_type> };
                let r_iter = quote! { <&'a #item_ty as itemize::IntoItems<#for_type>>::IntoIter };
                let m_fn = quote! { fn(&'a #item_ty) -> #r_iter };
                (iter, target, gens, conv, r_iter, m_fn)
            }
            CollectionType::Array => {
                let iter = quote! { ::std::array::IntoIter<#item_ty, N> };
                let target = quote! { [#item_ty; N] };
                let gens = quote! { <#item_ty, const N: usize, #for_generics> };
                let conv = quote! { #item_ty: itemize::IntoItems<#for_type> };
                let r_iter = quote! { <#item_ty as itemize::IntoItems<#for_type>>::IntoIter };
                let m_fn = quote! { fn(#item_ty) -> #r_iter };
                (iter, target, gens, conv, r_iter, m_fn)
            }
        };

        let rows = quote! { ::std::iter::Map<#into_iter, #map_fn> };

        let into_impl_base = match collection_type {
            CollectionType::Slice => quote! { self.iter() },
            _ => quote! { self.into_iter() },
        };

        let item_conversion = match collection_type {
            CollectionType::Slice => quote! { <&'a #item_ty as itemize::IntoItems<#for_type>>::into_items },
            _ => quote! { <#item_ty as itemize::IntoItems<#for_type>>::into_items },
        };

        let into_impl = quote! {
            #into_impl_base.map(#item_conversion)
        };

        Self {
            row_iter,
            rows,
            into_impl,
            target_ty,
            generics,
            conv_from,
        }
    }

    fn generate(self, ctx: &Context<'_>) -> TokenStream {
        let row_iter = self.row_iter;
        let rows = self.rows;
        let into_impl = self.into_impl;
        let generics = self.generics;
        let target_ty = self.target_ty;
        let conv_from = self.conv_from;

        let predicates = ctx.where_predicates.iter().flatten();
        let for_type = &ctx.for_type;

        quote! {
            impl #generics itemize::IntoRows<#for_type> for #target_ty
            where
                #(#predicates,)*
                #conv_from
            {
                type RowIter = #row_iter;
                type Rows = #rows;
                fn into_rows(self) -> Self::Rows {
                    #into_impl
                }
            }
        }
    }
}
