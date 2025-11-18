use quote::{ToTokens, format_ident};
use syn::parse::Parser;
use syn::{
    Attribute, DeriveInput, ExprArray, ImplGenerics, Meta, MetaList, MetaNameValue, TypeGenerics,
};

pub struct Context<'a> {
    pub attributes: Attributes,
    pub ident: &'a syn::Ident,
    pub trait_ident: syn::Ident,
    pub impl_generics: ImplGenerics<'a>,
    pub ty_generics: TypeGenerics<'a>,
    pub vis: &'a syn::Visibility,
}

impl<'a> Context<'a> {
    pub fn new(ast: &'a DeriveInput) -> Self {
        let ident = &ast.ident;
        let (impl_generics, ty_generics, _where_clause) = ast.generics.split_for_impl();
        let trait_ident = format_ident!("{}IntoItems", ident);
        Self {
            attributes: Attributes::from(&ast.attrs),
            ident,
            trait_ident,
            impl_generics,
            ty_generics,
            vis: &ast.vis,
        }
    }
    
    pub fn try_new(ast: &'a DeriveInput) -> syn::Result<Self> {
        let ident = &ast.ident;
        let (impl_generics, ty_generics, _where_clause) = ast.generics.split_for_impl();
        let trait_ident = format_ident!("{}IntoItems", ident);
        
        Ok(Self {
            attributes: Attributes::try_from(&ast.attrs)?,
            ident,
            trait_ident,
            impl_generics,
            ty_generics,
            vis: &ast.vis,
        })
    }
}

/// Example:
/// ```ignore
/// #[derive(IntoItems)]
/// #[into_items(from_types = [&str, String, i32, f64], from_tuples = 2)]
/// pub struct MySimpleType(String);
/// ```
#[derive(Default)]
pub struct Attributes {
    pub from_types: Vec<syn::Expr>,
    pub from_tuples: Option<usize>,
    pub from_collections: Vec<proc_macro2::TokenStream>,
}

impl Attributes {
    fn try_from(attrs: &Vec<Attribute>) -> syn::Result<Self> {
        let mut attributes = Attributes::default();

        for attr in attrs {
            if attr.path().is_ident("into_items") {
                let meta_items = attr.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )?;
                    for meta in meta_items {
                        match &meta {
                            // Handle `from_types = [...]` syntax
                            Meta::NameValue(MetaNameValue { path, value, .. })
                                if path.is_ident("from_types") =>
                            {
                                // Expect an array literal `[ ... ]`
                                let arr: ExprArray = syn::parse2(value.into_token_stream())
                                    .map_err(|_| syn::Error::new_spanned(
                                        value,
                                        "expected array literal like [Type1, Type2] for 'from_types'"
                                    ))?;
                                attributes.from_types = arr.elems.into_iter().collect();
                            }
                            // Handle `from_types(...)` syntax
                            Meta::List(MetaList { path, tokens, .. })
                                if path.is_ident("from_types") =>
                            {
                                // Parse the comma-separated types in parentheses
                                let types: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]> =
                                    syn::punctuated::Punctuated::parse_terminated
                                        .parse2(tokens.clone())
                                        .map_err(|_| syn::Error::new_spanned(
                                            path,
                                            "failed to parse types in from_types(...)"
                                        ))?;
                                attributes.from_types = types.into_iter().collect();
                            }
                            // Handle `from_tuples = 2` or `from_tuples = [2]` syntax
                            Meta::NameValue(MetaNameValue { path, value, .. })
                                if path.is_ident("from_tuples") =>
                            {
                                if let Ok(arr) = syn::parse2::<ExprArray>(value.into_token_stream())
                                {
                                    let mut elems = arr.elems.into_iter();
                                    if let Some(expr) = elems.next() {
                                        if let syn::Expr::Lit(syn::ExprLit {
                                            lit: syn::Lit::Int(lit_int),
                                            ..
                                        }) = expr
                                        {
                                            if let Ok(v) = lit_int.base10_parse::<usize>() {
                                                attributes.from_tuples = Some(v);
                                            }
                                        }
                                    }
                                } else {
                                    if let syn::Expr::Lit(syn::ExprLit {
                                        lit: syn::Lit::Int(lit_int),
                                        ..
                                    }) = syn::parse2::<syn::Expr>(value.into_token_stream())
                                        .map_err(|_| syn::Error::new_spanned(
                                            value,
                                            "expected integer for 'from_tuples'"
                                        ))?
                                    {
                                        if let Ok(v) = lit_int.base10_parse::<usize>() {
                                            attributes.from_tuples = Some(v);
                                        }
                                    }
                                }
                            }
                            // Handle `from_tuples(2)` syntax
                            Meta::List(MetaList { path, tokens, .. })
                                if path.is_ident("from_tuples") =>
                            {
                                // Parse as a single integer expression
                                if let Ok(expr) = syn::parse2::<syn::Expr>(tokens.clone()) {
                                    if let syn::Expr::Lit(syn::ExprLit {
                                        lit: syn::Lit::Int(lit_int),
                                        ..
                                    }) = expr
                                    {
                                        if let Ok(v) = lit_int.base10_parse::<usize>() {
                                            attributes.from_tuples = Some(v);
                                        }
                                    }
                                }
                            }
                            // Handle `from_collections(...)` syntax
                            Meta::List(MetaList { path, tokens, .. })
                                if path.is_ident("from_collections") =>
                            {
                                // Split the token stream by commas and store each collection type
                                let tokens_str = tokens.to_string();
                                let collection_types: Vec<proc_macro2::TokenStream> = tokens_str
                                    .split(',')
                                    .map(|s| {
                                        let trimmed = s.trim();
                                        trimmed.parse::<proc_macro2::TokenStream>().map_err(|_|
                                            syn::Error::new_spanned(
                                                path,
                                                format!("failed to parse collection type: {}", trimmed)
                                            )
                                        )
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;
                                attributes.from_collections = collection_types;
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &meta,
                                    "unknown attribute parameter; supported parameters are: from_types, from_tuples, from_collections"
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
