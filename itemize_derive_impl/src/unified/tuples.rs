use proc_macro2::TokenStream;
use quote::quote;

use crate::context::Context;
use crate::trait_config::TraitConfig;
use crate::util::{build_ordered_impl_generics, combine_where_clause, tuple_type_params};

pub fn from_tuples(context: &Context, config: &TraitConfig) -> TokenStream {
    let Some(max) = context.attributes.tuples else {
        return TokenStream::new();
    };

    let impls = (1..=max).map(|n| build_tuple_impl(context, config, n));
    quote! { #(#impls)* }
}

fn build_tuple_impl(context: &Context, config: &TraitConfig, len: usize) -> TokenStream {
    if config.kind == crate::trait_config::TraitKind::Items {
        build_items_tuple_impl(context, config, len)
    } else {
        build_rows_tuple_impl(context, config, len)
    }
}

fn build_items_tuple_impl(context: &Context, config: &TraitConfig, len: usize) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };
    let trait_path = config.trait_path();
    let trait_generics = config.trait_generics(&target);
    let method_name = config.method_name();

    let type_params = tuple_type_params("__TupleItem", len);
    let tuple_type = quote! { (#(#type_params,)*) };
    let item_type = config.iterator_item_type(&target);

    // Build chained conversion expressions
    let method_body = build_chained_conversions(config, &target, len);

    // Iterator type: Once for len=1, nested Chain<..., Once> for len>1
    let iterator_type = (1..len).fold(quote! { ::std::iter::Once<#item_type> }, |acc, _| {
        quote! { ::std::iter::Chain<#acc, ::std::iter::Once<#item_type>> }
    });

    // Build bounds
    let bounds: Vec<_> = if config.is_try() {
        let error_type = config.error_type_tokens();
        type_params
            .iter()
            .map(|ty| {
                quote! {
                    #target: ::std::convert::TryFrom<#ty>,
                    <#target as ::std::convert::TryFrom<#ty>>::Error: Into<#error_type>
                }
            })
            .collect()
    } else {
        type_params
            .iter()
            .map(|ty| quote! { #target: ::std::convert::From<#ty> })
            .collect()
    };

    let extra_generics: Vec<_> = type_params.iter().map(|p| quote! { #p }).collect();
    let impl_generics = build_ordered_impl_generics(context, config, &extra_generics);
    let where_clause = combine_where_clause(context, bounds);

    quote! {
        impl #impl_generics #trait_path<#trait_generics> for #tuple_type #where_clause
        {
            type IntoIter = #iterator_type;
            fn #method_name(self) -> Self::IntoIter {
                #method_body
            }
        }
    }
}

fn build_chained_conversions(config: &TraitConfig, target: &TokenStream, len: usize) -> TokenStream {
    let conversions: Vec<_> = (0..len)
        .map(|i| {
            let index = syn::Index::from(i);
            let conversion = if config.is_try() {
                let error_type = config.error_type_tokens();
                quote! { <#target>::try_from(self.#index).map_err(Into::<#error_type>::into) }
            } else {
                config.wrap_conversion(quote! { <#target>::from(self.#index) })
            };

            if i == 0 {
                quote! { ::std::iter::once(#conversion) }
            } else {
                quote! { .chain(::std::iter::once(#conversion)) }
            }
        })
        .collect();

    quote! { #(#conversions)* }
}

fn build_rows_tuple_impl(context: &Context, config: &TraitConfig, len: usize) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };
    let trait_path = config.trait_path();
    let trait_generics = config.trait_generics(&target);
    let method_name = config.method_name();

    let type_params = tuple_type_params("__RowItem", len);
    let tuple_type = quote! { (#(#type_params,)*) };

    let row_iter_item = config.iterator_item_type(&target);
    let row_iter_ty = quote! { Box<dyn std::iter::Iterator<Item = #row_iter_item>> };
    let rows_type = quote! { std::array::IntoIter<#row_iter_ty, #len> };

    let (into_items_trait, into_items_method) = if config.is_try() {
        let error_type = config.error_type_tokens();
        (
            quote! { itemize::TryIntoItems<#target, #error_type> },
            quote! { try_into_items },
        )
    } else {
        (quote! { itemize::IntoItems<#target> }, quote! { into_items })
    };

    let array_elements: Vec<_> = (0..len)
        .map(syn::Index::from)
        .map(|idx| quote! { Box::new(self.#idx.#into_items_method()) as #row_iter_ty })
        .collect();

    let bounds: Vec<_> = type_params
        .iter()
        .flat_map(|ty| {
            let trait_bound = quote! { #ty: #into_items_trait };
            let iter_bound = if config.is_try() {
                let error_type = config.error_type_tokens();
                quote! { <#ty as itemize::TryIntoItems<#target, #error_type>>::IntoIter: 'static }
            } else {
                quote! { <#ty as itemize::IntoItems<#target>>::IntoIter: 'static }
            };
            [trait_bound, iter_bound]
        })
        .collect();

    let extra_generics: Vec<_> = type_params.iter().map(|p| quote! { #p }).collect();
    let impl_generics = build_ordered_impl_generics(context, config, &extra_generics);
    let where_clause = combine_where_clause(context, bounds);

    quote! {
        impl #impl_generics #trait_path<#trait_generics> for #tuple_type #where_clause
        {
            type RowIter = #row_iter_ty;
            type Rows = #rows_type;
            fn #method_name(self) -> Self::Rows {
                [ #(#array_elements,)* ].into_iter()
            }
        }
    }
}
