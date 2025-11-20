use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, format_ident, quote};

use crate::context::Context;

pub(crate) fn impl_generics_tokens(context: &Context, extra: &[TokenStream]) -> TokenStream {
    let mut params: Vec<TokenStream> = extra.to_vec();
    for param in context.generics.params.iter() {
        params.push(param.to_token_stream());
    }

    if params.is_empty() {
        TokenStream::new()
    } else {
        quote! { <#(#params),*> }
    }
}

pub(crate) fn tuple_type_params(prefix: &str, len: usize) -> Vec<Ident> {
    (0..len)
        .map(|index| format_ident!("{}{}", prefix, index))
        .collect()
}

pub(crate) fn tuple_vars(prefix: &str, len: usize) -> Vec<Ident> {
    tuple_type_params(prefix, len)
}

pub(crate) fn combine_where_clause(
    context: &Context,
    extra_predicates: impl IntoIterator<Item = TokenStream>,
) -> TokenStream {
    let mut predicates: Vec<TokenStream> = context
        .where_clause
        .map(|clause| {
            clause
                .predicates
                .iter()
                .map(|pred| pred.to_token_stream())
                .collect()
        })
        .unwrap_or_default();

    predicates.extend(extra_predicates);

    if predicates.is_empty() {
        TokenStream::new()
    } else {
        quote! { where #(#predicates,)* }
    }
}

pub(crate) fn map_item_fn_tokens() -> TokenStream {
    quote! {
        fn map_item<Target, Item>(item: Item) -> Target
        where
            Target: ::std::convert::From<Item>,
        {
            Target::from(item)
        }
    }
}

pub(crate) fn map_row_fn_tokens() -> TokenStream {
    quote! {
        fn map_row<Target, Item>(item: Item) -> <Item as itemize::IntoItems<Target>>::IntoIter
        where
            Item: itemize::IntoItems<Target>,
        {
            item.into_items()
        }
    }
}
