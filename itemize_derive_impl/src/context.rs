use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use syn::{Attribute, DeriveInput, ImplGenerics, Meta, MetaList, TypeGenerics, WhereClause};

pub struct Context<'a> {
    pub attributes: Attributes,
    pub ident: &'a syn::Ident,
    pub impl_generics: ImplGenerics<'a>,
    pub ty_generics: TypeGenerics<'a>,
    pub where_clause: Option<&'a WhereClause>,
    pub vis: &'a syn::Visibility,
    pub generics: &'a syn::Generics,
}

impl<'a> Context<'a> {
    pub fn try_new(ast: &'a DeriveInput) -> syn::Result<Self> {
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
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        Ok(Self {
            attributes: Attributes::try_from(&ast.attrs)?,
            ident,
            impl_generics,
            ty_generics,
            where_clause,
            vis: &ast.vis,
            generics: &ast.generics,
        })
    }
}

/// Example:
/// ```ignore
/// #[items_from(types = [String, char], tuples = 2, collections = [vec, slice, array])]
/// #[items_from(types(String, char), tuples(2), collections(vec, slice, array))]
/// ```
#[derive(Default)]
pub struct Attributes {
    pub types: Vec<syn::Type>,
    pub tuples: Option<usize>,
    pub collections: HashSet<CollectionType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionType {
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
    const COLLECTIONS_IDENT: &str = "collections";

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

                        // Handle `tuples(2)` syntax
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
                        _ => {
                            return Err(syn::Error::new_spanned(
                                &meta,
                                "unknown attribute parameter; supported parameters are: types, tuples, collections",
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

    fn parse_tuples(tokens: &TokenStream) -> syn::Result<usize> {
        let lit: syn::LitInt = syn::parse2(tokens.clone())
            .map_err(|_| err(tokens, "expected integer for `tuples`"))?;
        lit.base10_parse()
            .map_err(|_| err(tokens, "invalid integer for `tuples`"))
    }

    fn parse_collections(tokens: &TokenStream) -> syn::Result<Vec<CollectionType>> {
        let idents: syn::punctuated::Punctuated<syn::Ident, syn::Token![,]> =
            syn::punctuated::Punctuated::parse_terminated.parse2(tokens.clone())?;

        idents
            .into_iter()
            .map(|ident| CollectionType::try_from(ident))
            .collect::<Result<Vec<_>, _>>()
    }
}

fn err<T: ToTokens, U: std::fmt::Display>(tokens: T, message: U) -> syn::Error {
    syn::Error::new_spanned(tokens, message)
}
