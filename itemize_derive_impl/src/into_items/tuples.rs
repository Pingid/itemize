use proc_macro2::TokenStream;
use quote::quote;

use crate::context::Context;
use crate::util::{combine_where_clause, impl_generics_tokens, tuple_type_params, tuple_vars};

pub fn from_tuples(context: &Context) -> TokenStream {
    let Some(max) = context.attributes.tuples else {
        return TokenStream::new();
    };

    let mut expanded = TokenStream::new();

    for len in 1..=max {
        expanded.extend(build_tuple_impl(context, len));
    }

    expanded
}

fn build_tuple_impl(context: &Context, len: usize) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = if context.generics.params.is_empty() {
        quote! { #ident }
    } else {
        quote! { #ident #ty_generics }
    };

    let type_params = tuple_type_params("__TupleItem", len);
    let vars = tuple_vars("a", len);

    let tuple_type = quote! { (#(#type_params,)*) };
    let iter_type = quote! { std::array::IntoIter<#target, #len> };

    let extra_generics: Vec<TokenStream> =
        type_params.iter().map(|ident| quote! { #ident }).collect();
    let impl_generics = impl_generics_tokens(context, &extra_generics);

    let where_predicates = type_params
        .iter()
        .map(|ty| quote! { #target: From<#ty> })
        .collect::<Vec<_>>();
    let where_clause = combine_where_clause(context, where_predicates);

    let conversions = vars
        .iter()
        .map(|var| quote! { #ident::from(#var) })
        .collect::<Vec<_>>();

    quote! {
        impl #impl_generics itemize::IntoItems<#target> for #tuple_type #where_clause {
            type IntoIter = #iter_type;

            fn into_items(self) -> Self::IntoIter {
                let (#(#vars,)*) = self;
                <[#target; #len]>::into_iter([#(#conversions),*])
            }
        }
    }
}
