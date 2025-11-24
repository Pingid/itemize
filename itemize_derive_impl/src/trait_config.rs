use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitVariant {
    Regular,
    Try,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraitKind {
    Items,
    Rows,
}

pub struct TraitConfig {
    pub variant: TraitVariant,
    pub kind: TraitKind,
    pub error_type: Option<syn::Type>,
}

impl TraitConfig {
    pub fn new(variant: TraitVariant, kind: TraitKind) -> Self {
        Self {
            variant,
            kind,
            error_type: None,
        }
    }

    pub fn with_error_type(mut self, error_type: Option<syn::Type>) -> Self {
        self.error_type = error_type;
        self
    }

    pub fn trait_name(&self) -> Ident {
        match (self.variant, self.kind) {
            (TraitVariant::Regular, TraitKind::Items) => format_ident!("IntoItems"),
            (TraitVariant::Regular, TraitKind::Rows) => format_ident!("IntoRows"),
            (TraitVariant::Try, TraitKind::Items) => format_ident!("TryIntoItems"),
            (TraitVariant::Try, TraitKind::Rows) => format_ident!("TryIntoRows"),
        }
    }

    pub fn method_name(&self) -> Ident {
        match (self.variant, self.kind) {
            (TraitVariant::Regular, TraitKind::Items) => format_ident!("into_items"),
            (TraitVariant::Regular, TraitKind::Rows) => format_ident!("into_rows"),
            (TraitVariant::Try, TraitKind::Items) => format_ident!("try_into_items"),
            (TraitVariant::Try, TraitKind::Rows) => format_ident!("try_into_rows"),
        }
    }

    pub fn trait_path(&self) -> TokenStream {
        let trait_name = self.trait_name();
        quote! { itemize::#trait_name }
    }

    pub fn trait_generics(&self, target: &TokenStream) -> TokenStream {
        match self.variant {
            TraitVariant::Regular => quote! { #target },
            TraitVariant::Try => {
                let error_type = self.error_type_tokens();
                quote! { #target, #error_type }
            }
        }
    }

    pub fn error_type_tokens(&self) -> TokenStream {
        self.error_type
            .as_ref()
            .map(|ty| quote! { #ty })
            .unwrap_or_else(|| quote! { __E })
    }

    pub fn wrap_conversion(&self, expr: TokenStream) -> TokenStream {
        match self.variant {
            TraitVariant::Regular => expr,
            TraitVariant::Try => quote! { Ok(#expr) },
        }
    }

    pub fn is_try(&self) -> bool {
        self.variant == TraitVariant::Try
    }

    pub fn iterator_item_type(&self, item_type: &TokenStream) -> TokenStream {
        match self.variant {
            TraitVariant::Regular => quote! { #item_type },
            TraitVariant::Try => {
                let error_type = self.error_type_tokens();
                quote! { Result<#item_type, #error_type> }
            }
        }
    }

    pub fn needs_error_generic(&self) -> bool {
        self.variant == TraitVariant::Try && self.error_type.is_none()
    }

    pub fn associated_types(&self, impl_types: AssociatedTypes) -> TokenStream {
        match self.kind {
            TraitKind::Items => {
                let into_iter = impl_types.into_iter;
                quote! {
                    type IntoIter = #into_iter;
                }
            }
            TraitKind::Rows => {
                let row_iter = impl_types.row_iter;
                let rows = impl_types.rows;
                quote! {
                    type RowIter = #row_iter;
                    type Rows = #rows;
                }
            }
        }
    }
}

pub struct AssociatedTypes {
    pub into_iter: TokenStream,
    pub row_iter: Option<TokenStream>,
    pub rows: Option<TokenStream>,
}

impl AssociatedTypes {
    pub fn for_items(into_iter: TokenStream) -> Self {
        Self {
            into_iter,
            row_iter: None,
            rows: None,
        }
    }

    pub fn for_rows(row_iter: TokenStream, rows: TokenStream) -> Self {
        Self {
            into_iter: TokenStream::new(),
            row_iter: Some(row_iter),
            rows: Some(rows),
        }
    }
}
