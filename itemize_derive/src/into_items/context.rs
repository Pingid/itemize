use quote::ToTokens;
use syn::parse::Parser;
use syn::{
    Attribute, DeriveInput, ExprArray, ImplGenerics, Meta, MetaList, MetaNameValue, TypeGenerics,
    WhereClause,
};

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
/// #[derive(IntoItems)]
/// #[into_items(types = [&str, String, i32, f64], tuples = 2)]
/// pub struct MySimpleType(String);
/// ```
#[derive(Default)]
pub struct Attributes {
    pub types: Vec<syn::Expr>,
    pub tuples: Option<usize>,
    pub collections: Vec<proc_macro2::TokenStream>,
}

impl Attributes {
    fn try_from(attrs: &Vec<Attribute>) -> syn::Result<Self> {
        let mut attributes = Attributes::default();

        for attr in attrs {
            if attr.path().is_ident("items_from") {
                let meta_items = attr.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )?;
                for meta in meta_items {
                    match &meta {
                        // Handle `types = [...]` syntax
                        Meta::NameValue(MetaNameValue { path, value, .. })
                            if path.is_ident("types") =>
                        {
                            // Expect an array literal `[ ... ]`
                            let arr: ExprArray =
                                syn::parse2(value.into_token_stream()).map_err(|_| {
                                    syn::Error::new_spanned(
                                        value,
                                        "expected array literal like [Type1, Type2] for 'types'",
                                    )
                                })?;
                            attributes.types = arr.elems.into_iter().collect();
                        }
                        // Handle `types(...)` syntax
                        Meta::List(MetaList { path, tokens, .. }) if path.is_ident("types") => {
                            // Parse the comma-separated types in parentheses
                            let types: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]> =
                                syn::punctuated::Punctuated::parse_terminated
                                    .parse2(tokens.clone())
                                    .map_err(|_| {
                                        syn::Error::new_spanned(
                                            path,
                                            "failed to parse types in types(...)",
                                        )
                                    })?;
                            attributes.types = types.into_iter().collect();
                        }
                        // Handle `tuples = 2` or `tuples = [2]` syntax
                        Meta::NameValue(MetaNameValue { path, value, .. })
                            if path.is_ident("tuples") =>
                        {
                            if let Ok(arr) = syn::parse2::<ExprArray>(value.into_token_stream()) {
                                let mut elems = arr.elems.into_iter();
                                if let Some(expr) = elems.next() {
                                    if let syn::Expr::Lit(syn::ExprLit {
                                        lit: syn::Lit::Int(lit_int),
                                        ..
                                    }) = expr
                                    {
                                        if let Ok(v) = lit_int.base10_parse::<usize>() {
                                            attributes.tuples = Some(v);
                                        }
                                    }
                                }
                            } else {
                                if let syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Int(lit_int),
                                    ..
                                }) = syn::parse2::<syn::Expr>(value.into_token_stream())
                                    .map_err(|_| {
                                        syn::Error::new_spanned(
                                            value,
                                            "expected integer for 'tuples'",
                                        )
                                    })?
                                {
                                    if let Ok(v) = lit_int.base10_parse::<usize>() {
                                        attributes.tuples = Some(v);
                                    }
                                }
                            }
                        }
                        // Handle `tuples(2)` syntax
                        Meta::List(MetaList { path, tokens, .. }) if path.is_ident("tuples") => {
                            // Parse as a single integer expression
                            if let Ok(expr) = syn::parse2::<syn::Expr>(tokens.clone()) {
                                if let syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Int(lit_int),
                                    ..
                                }) = expr
                                {
                                    if let Ok(v) = lit_int.base10_parse::<usize>() {
                                        attributes.tuples = Some(v);
                                    }
                                }
                            }
                        }
                        // Handle `collections(...)` syntax
                        Meta::List(MetaList { path, tokens, .. })
                            if path.is_ident("collections") =>
                        {
                            // Split the token stream by commas and store each collection type
                            let tokens_str = tokens.to_string();
                            let collection_types: Vec<proc_macro2::TokenStream> = tokens_str
                                .split(',')
                                .map(|s| {
                                    let trimmed = s.trim();
                                    trimmed.parse::<proc_macro2::TokenStream>().map_err(|_| {
                                        syn::Error::new_spanned(
                                            path,
                                            format!("failed to parse collection type: {}", trimmed),
                                        )
                                    })
                                })
                                .collect::<Result<Vec<_>, _>>()?;
                            attributes.collections = collection_types;
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
}

impl From<&Vec<Attribute>> for Attributes {
    fn from(attrs: &Vec<Attribute>) -> Self {
        // Fallback implementation that panics on errors for backward compatibility
        Self::try_from(attrs).expect("Failed to parse attributes")
    }
}
