use proc_macro2::TokenStream;
use quote::quote;

use crate::context::Context;
use crate::util::{combine_where_clause, impl_generics_tokens, tuple_type_params};

pub fn from_tuples(context: &Context) -> TokenStream {
    let Some(max) = context.attributes.tuples else {
        return TokenStream::new();
    };

    let mut expanded = TokenStream::new();

    for n in 2..=max {
        expanded.extend(build_tuple_impl(context, n));
    }

    expanded
}

fn build_tuple_impl(context: &Context, len: usize) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };

    let type_params = tuple_type_params("__RowItem", len);
    let indices: Vec<_> = (0..len).map(syn::Index::from).collect();

    let tuple_type = quote! { (#(#type_params,)*) };
    let row_iter_ty = quote! { Box<dyn std::iter::Iterator<Item = #target>> };
    let rows_type = quote! { std::array::IntoIter<#row_iter_ty, #len> };

    let array_elements: Vec<_> = indices
        .iter()
        .map(|idx| quote! { Box::new(self.#idx.into_items()) as #row_iter_ty })
        .collect();

    let tuple_bounds: Vec<_> = type_params
        .iter()
        .map(|ty| quote! { #ty: itemize::IntoItems<#target> })
        .collect();
    let iterator_bounds: Vec<_> = type_params
        .iter()
        .map(|ty| quote! { <#ty as itemize::IntoItems<#target>>::IntoIter: 'static })
        .collect();

    let extra_generics: Vec<TokenStream> =
        type_params.iter().map(|ident| quote! { #ident }).collect();
    let impl_generics = impl_generics_tokens(context, &extra_generics);
    let where_clause = combine_where_clause(
        context,
        tuple_bounds.iter().chain(iterator_bounds.iter()).cloned(),
    );

    quote! {
        impl #impl_generics itemize::IntoRows<#target> for #tuple_type #where_clause
        {
            type RowIter = #row_iter_ty;
            type Rows = #rows_type;

            fn into_rows(self) -> Self::Rows {
                [
                    #(#array_elements,)*
                ].into_iter()
            }
        }
    }
}
