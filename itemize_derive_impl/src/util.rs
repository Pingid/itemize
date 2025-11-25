use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Generics, Ident, Lifetime};

pub(crate) fn tuple_type_ident(len: usize) -> Ident {
    format_ident!("__A{}", len)
}

pub(crate) fn item_ident() -> Ident {
    format_ident!("__T")
}

pub(crate) fn const_ident() -> Ident {
    format_ident!("__N")
}

pub(crate) fn tuple_items_impl(
    len: usize,
    f: impl Fn(Ident) -> TokenStream,
    wrap: impl Fn(TokenStream) -> TokenStream,
) -> TokenStream {
    let names = tuple_names(len);
    let block = names
        .iter()
        .cloned()
        .map(f)
        .fold(quote! {}, |acc, x| match acc.is_empty() {
            true => x,
            false => quote! { #acc, #x },
        });

    let body = wrap(block);
    let destructure = tuple_destructure(&names);
    quote! {
        #destructure
        #body
    }
}

/// Generates body for tuple IntoRows/TryIntoRows with Either wrapping.
pub(crate) fn tuple_rows_impl(len: usize, f: impl Fn(Ident) -> TokenStream) -> TokenStream {
    let names = tuple_names(len);
    let destructure = tuple_destructure(&names);

    let exprs = names.iter().enumerate().map(|(i, name)| {
        let base = f(name.clone());
        either_val(i, len, base)
    });

    quote! {
        use itemize::either::Either::*;
        #destructure
        [#(#exprs),*].into_iter()
    }
}

pub(crate) fn tuple_rows_associated(
    len: usize,
    iter_types: &[TokenStream],
    for_type: impl ToTokens,
) -> TokenStream {
    match len {
        0 => quote! { ::std::iter::Empty<#for_type> },
        1 => iter_types[0].clone(),
        _ => either_type(iter_types),
    }
}

/// Builds a nested Either type from a list of types.
fn either_type(types: &[TokenStream]) -> TokenStream {
    match types.len() {
        0 => panic!("either_type requires at least one type"),
        1 => types[0].clone(),
        _ => {
            let head = &types[0];
            let tail = either_type(&types[1..]);
            quote! { itemize::Either<#head, #tail> }
        }
    }
}

fn either_val(idx: usize, len: usize, expr: TokenStream) -> TokenStream {
    if len == 1 {
        expr
    } else if idx == 0 {
        quote! { Left(#expr) }
    } else {
        let inner = either_val(idx - 1, len - 1, expr);
        quote! { Right(#inner) }
    }
}

fn tuple_names(len: usize) -> Vec<Ident> {
    (0..len).map(|i| format_ident!("a{}", i)).collect()
}

fn tuple_destructure(names: &[Ident]) -> TokenStream {
    match names.len() {
        0 => quote! {},
        1 => {
            let n = &names[0];
            quote! { let (#n,) = self; }
        }
        _ => quote! { let (#(#names),*) = self; },
    }
}

pub(crate) struct GenericList {
    params: Vec<GenericParam>,
}

#[derive(Clone)]
pub(crate) enum GenericParam {
    Lifetime(TokenStream),
    Const(TokenStream),
    Type(TokenStream),
}

impl ToTokens for GenericParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            GenericParam::Lifetime(p) => p.to_tokens(tokens),
            GenericParam::Const(p) => p.to_tokens(tokens),
            GenericParam::Type(p) => p.to_tokens(tokens),
        }
    }
}

impl GenericList {
    pub(crate) fn new() -> Self {
        Self { params: Vec::new() }
    }

    pub(crate) fn with_types<T: ToTokens>(mut self, ts: impl IntoIterator<Item = T>) -> Self {
        for t in ts {
            self.params.push(GenericParam::Type(t.to_token_stream()));
        }
        self
    }

    pub(crate) fn with_lifetimes<T: ToTokens>(mut self, ts: impl IntoIterator<Item = T>) -> Self {
        for t in ts {
            self.params
                .push(GenericParam::Lifetime(t.to_token_stream()));
        }
        self
    }

    pub(crate) fn with_consts<T: ToTokens>(mut self, ts: impl IntoIterator<Item = T>) -> Self {
        for t in ts {
            self.params.push(GenericParam::Const(t.to_token_stream()));
        }
        self
    }

    pub(crate) fn with_generics(mut self, generics: &Generics) -> Self {
        for t in &generics.params {
            match t {
                syn::GenericParam::Lifetime(lifetime) => self
                    .params
                    .push(GenericParam::Lifetime(lifetime.to_token_stream())),
                syn::GenericParam::Const(const_) => self
                    .params
                    .push(GenericParam::Const(const_.to_token_stream())),
                syn::GenericParam::Type(type_) => self
                    .params
                    .push(GenericParam::Type(type_.to_token_stream())),
            }
        }
        self
    }

    pub(crate) fn with_lifetimes_from_type(mut self, ty: &syn::Type) -> Self {
        let mut seen: HashSet<String> = self
            .params
            .iter()
            .filter_map(|p| match p {
                // Extract lifetime name, handling bounds like "'a: 'static" -> "'a"
                GenericParam::Lifetime(ts) => ts
                    .to_string()
                    .split_whitespace()
                    .next()
                    .map(|s| s.to_string()),
                _ => None,
            })
            .collect();

        for lifetime in extract_lifetimes_types(ty) {
            if seen.insert(lifetime.to_string()) {
                self.params
                    .push(GenericParam::Lifetime(lifetime.to_token_stream()));
            }
        }
        self
    }
}

impl ToTokens for GenericList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params = self.params.clone();
        params.sort_by_key(|param| match param {
            GenericParam::Lifetime(_) => 0,
            GenericParam::Const(_) => 1,
            GenericParam::Type(_) => 2,
        });
        if !params.is_empty() {
            tokens.extend(quote! { <#(#params),*> })
        }
    }
}

pub(crate) fn extract_lifetimes_types(ty: &syn::Type) -> Vec<&Lifetime> {
    let mut lifetimes = Vec::new();

    match ty {
        syn::Type::Reference(ref_ty) => {
            if let Some(lifetime) = &ref_ty.lifetime {
                lifetimes.push(lifetime);
            }
            lifetimes.extend(extract_lifetimes_types(&ref_ty.elem));
        }
        syn::Type::Path(path_ty) => {
            if let Some(qself) = &path_ty.qself {
                lifetimes.extend(extract_lifetimes_types(&qself.ty));
            }
            for segment in &path_ty.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Lifetime(lt) => lifetimes.push(lt),
                            syn::GenericArgument::Type(ty) => {
                                lifetimes.extend(extract_lifetimes_types(ty))
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        syn::Type::Tuple(tuple_ty) => {
            for elem in &tuple_ty.elems {
                lifetimes.extend(extract_lifetimes_types(elem));
            }
        }
        syn::Type::Array(array_ty) => {
            lifetimes.extend(extract_lifetimes_types(&array_ty.elem));
        }
        syn::Type::Slice(slice_ty) => {
            lifetimes.extend(extract_lifetimes_types(&slice_ty.elem));
        }
        _ => {}
    }

    lifetimes
}
