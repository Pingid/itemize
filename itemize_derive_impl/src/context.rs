use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::Parser;
use syn::{Attribute, DeriveInput, Meta, MetaList};

use crate::util::GenericList;

pub(crate) struct Context<'a> {
    pub(crate) attributes: Attributes,
    pub(crate) where_predicates: Option<Vec<TokenStream>>,
    pub(crate) generics: &'a syn::Generics,
    pub(crate) concrete: TokenStream,
}

impl<'a> Context<'a> {
    pub(crate) fn try_new(ast: &'a DeriveInput) -> syn::Result<Self> {
        // Validate that this is being used on a struct or enum
        match &ast.data {
            syn::Data::Struct(_) | syn::Data::Enum(_) => {}
            syn::Data::Union(_) => {
                return Err(syn::Error::new_spanned(
                    &ast.ident,
                    "Items cannot be derived for unions",
                ));
            }
        }

        let ident = &ast.ident;
        let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

        let concrete = quote! { #ident #ty_generics };
        let where_predicates = where_clause.map(|clause| {
            clause
                .predicates
                .iter()
                .map(|pred| pred.to_token_stream())
                .collect::<Vec<_>>()
        });
        Ok(Self {
            attributes: Attributes::try_from(&ast.attrs)?,
            generics: &ast.generics,
            where_predicates,
            concrete,
        })
    }

    pub(crate) fn generics(&self) -> GenericList {
        GenericList::new().with_generics(self.generics)
    }

    pub(crate) fn error_generics(&self) -> GenericList {
        match &self.attributes.error_type {
            Some(_) => self.generics(),
            None => self.generics().with_types(self.error_ty()),
        }
    }

    pub(crate) fn error_ty(&self) -> TokenStream {
        match &self.attributes.error_type {
            Some(ty) => quote! { #ty },
            None => quote! { E },
        }
    }
}

/// Example:
/// ```ignore
/// #[items_from(types(String, char), tuples, collections(vec, slice, array))]
/// #[items_from(tuples(1..=4))]  // explicit range
/// #[items_from(tuples(2..=4))]  // excludes 1-tuples
/// #[items_from(tuples(4))]      // shorthand for 1..=4
/// #[items_from(tuples(exact(4)))] // only size 4
/// #[items_from(error_type(MyError))]
/// ```
#[derive(Default)]
pub(crate) struct Attributes {
    pub types: Vec<syn::Type>,
    pub tuples: Option<TupleRange>,
    pub collections: HashSet<CollectionType>,
    pub error_type: Option<syn::Type>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct TupleRange {
    pub start: usize,
    pub end: usize,
}

impl TupleRange {
    pub fn iter(self) -> std::ops::RangeInclusive<usize> {
        self.start..=self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum CollectionType {
    Vec,
    Slice,
    Array,
}

impl TryFrom<syn::Ident> for CollectionType {
    type Error = syn::Error;
    fn try_from(ident: syn::Ident) -> Result<Self, Self::Error> {
        if ident == "vec" {
            Ok(CollectionType::Vec)
        } else if ident == "slice" {
            Ok(CollectionType::Slice)
        } else if ident == "array" {
            Ok(CollectionType::Array)
        } else {
            Err(err(
                ident,
                "invalid collection type: expected one of `vec`, `slice`, `array`",
            ))
        }
    }
}

impl Attributes {
    const PATH_IDENT: &str = "items_from";
    const TYPES_IDENT: &str = "types";
    const TUPLES_IDENT: &str = "tuples";
    const DEFAULT_TUPLES: TupleRange = TupleRange { start: 1, end: 6 };
    const COLLECTIONS_IDENT: &str = "collections";
    const ERROR_TYPE_IDENT: &str = "error_type";

    fn try_from(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut attributes = Attributes::default();

        for attr in attrs {
            if attr.path().is_ident(Self::PATH_IDENT) {
                let meta_items = attr.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )?;
                for meta in meta_items {
                    match &meta {
                        // Handle `types(...)` syntax
                        Meta::List(MetaList { path, tokens, .. })
                            if path.is_ident(Self::TYPES_IDENT) =>
                        {
                            attributes.types = Self::parse_types(tokens)?;
                        }

                        // Handle `tuples` or `tuples(N)` syntax
                        Meta::Path(path) if path.is_ident(Self::TUPLES_IDENT) => {
                            attributes.tuples = Some(Self::DEFAULT_TUPLES);
                        }
                        Meta::List(MetaList { path, tokens, .. })
                            if path.is_ident(Self::TUPLES_IDENT) =>
                        {
                            attributes.tuples = Some(Self::parse_tuples(tokens)?);
                        }

                        // Handle `collections(...)` syntax
                        Meta::List(MetaList { path, tokens, .. })
                            if path.is_ident(Self::COLLECTIONS_IDENT) =>
                        {
                            attributes
                                .collections
                                .extend(Self::parse_collections(tokens)?);
                        }

                        // Handle `error_type(...)` syntax
                        Meta::List(MetaList { path, tokens, .. })
                            if path.is_ident(Self::ERROR_TYPE_IDENT) =>
                        {
                            attributes.error_type = Some(Self::parse_error_type(tokens)?);
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &meta,
                                "unknown attribute parameter; supported parameters are: types, tuples, collections, error_type",
                            ));
                        }
                    }
                }
            }
        }

        Ok(attributes)
    }

    fn parse_types(tokens: &TokenStream) -> syn::Result<Vec<syn::Type>> {
        let types: syn::punctuated::Punctuated<syn::Type, syn::Token![,]> =
            syn::punctuated::Punctuated::parse_terminated
                .parse2(tokens.clone())
                .map_err(|_| err(tokens, "failed to parse types"))?;
        Ok(types.into_iter().collect())
    }

    fn parse_tuples(tokens: &TokenStream) -> syn::Result<TupleRange> {
        syn::parse::Parser::parse2(Self::parse_tuple_range, tokens.clone())
    }

    fn parse_tuple_range(input: syn::parse::ParseStream) -> syn::Result<TupleRange> {
        // Try parsing as exact(N)
        if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse()?;
            if ident == "exact" {
                let content;
                syn::parenthesized!(content in input);
                let lit: syn::LitInt = content.parse()?;
                let n: usize = lit.base10_parse()?;
                return Ok(TupleRange { start: n, end: n });
            }
            return Err(syn::Error::new_spanned(ident, "expected `exact`"));
        }

        // Try parsing as range (1..=4) or shorthand (4)
        let start: syn::LitInt = input.parse()?;
        let start_val: usize = start.base10_parse()?;

        // Check for range syntax
        if input.peek(syn::Token![..]) {
            input.parse::<syn::Token![..]>()?;
            input.parse::<syn::Token![=]>()?;
            let end: syn::LitInt = input.parse()?;
            let end_val: usize = end.base10_parse()?;
            Ok(TupleRange {
                start: start_val,
                end: end_val,
            })
        } else {
            // Shorthand: N means 1..=N
            Ok(TupleRange {
                start: 1,
                end: start_val,
            })
        }
    }

    fn parse_collections(tokens: &TokenStream) -> syn::Result<Vec<CollectionType>> {
        let idents: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
            syn::punctuated::Punctuated::parse_terminated.parse2(tokens.clone())?;

        idents
            .into_iter()
            .map(CollectionType::try_from)
            .collect::<Result<Vec<_>, _>>()
    }

    fn parse_error_type(tokens: &TokenStream) -> syn::Result<syn::Type> {
        syn::parse2(tokens.clone()).map_err(|_| err(tokens, "expected type for `error_type`"))
    }
}

fn err<T: ToTokens, U: std::fmt::Display>(tokens: T, message: U) -> syn::Error {
    syn::Error::new_spanned(tokens, message)
}
