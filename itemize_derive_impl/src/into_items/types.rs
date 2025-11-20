use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{GenericParam, Lifetime};

use crate::context::Context;
use crate::util::combine_where_clause;

/// Extract lifetimes from a type
fn extract_lifetimes(ty: &syn::Type) -> Vec<Lifetime> {
    let mut lifetimes = Vec::new();
    
    match ty {
        syn::Type::Reference(ref_ty) => {
            if let Some(lifetime) = &ref_ty.lifetime {
                lifetimes.push(lifetime.clone());
            }
            // Recursively check the inner type
            lifetimes.extend(extract_lifetimes(&ref_ty.elem));
        }
        syn::Type::Path(path_ty) => {
            // Check generic arguments for lifetimes
            if let Some(qself) = &path_ty.qself {
                lifetimes.extend(extract_lifetimes(&qself.ty));
            }
            for segment in &path_ty.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Lifetime(lt) = arg {
                            lifetimes.push(lt.clone());
                        } else if let syn::GenericArgument::Type(ty) = arg {
                            lifetimes.extend(extract_lifetimes(ty));
                        }
                    }
                }
            }
        }
        syn::Type::Tuple(tuple_ty) => {
            for elem in &tuple_ty.elems {
                lifetimes.extend(extract_lifetimes(elem));
            }
        }
        syn::Type::Array(array_ty) => {
            lifetimes.extend(extract_lifetimes(&array_ty.elem));
        }
        syn::Type::Slice(slice_ty) => {
            lifetimes.extend(extract_lifetimes(&slice_ty.elem));
        }
        _ => {}
    }
    
    lifetimes
}

pub fn from_type(context: &Context) -> TokenStream {
    let ident = &context.ident;
    let from_types = &context.attributes.types;
    let ty_generics = &context.ty_generics;

    // Check if the struct is generic
    if context.generics.params.is_empty() {
        // Non-generic case
        // For each from_type, extract lifetimes and add them to the impl block
        let expanded_impls = from_types.iter().map(|from_type| {
            let lifetimes = extract_lifetimes(from_type);
            let unique_lifetimes: HashSet<_> = lifetimes.iter().cloned().collect();
            let lifetime_params: Vec<GenericParam> = unique_lifetimes
                .into_iter()
                .map(|lt| GenericParam::Lifetime(syn::LifetimeParam::new(lt)))
                .collect();
            
            if lifetime_params.is_empty() {
                // No lifetimes in the type
                quote! {
                    impl itemize::IntoItems<#ident> for #from_type {
                        type IntoIter = ::std::iter::Once<#ident>;
                        fn into_items(self) -> Self::IntoIter {
                            ::std::iter::once(#ident::from(self))
                        }
                    }
                }
            } else {
                // Has lifetimes
                quote! {
                    impl<#(#lifetime_params),*> itemize::IntoItems<#ident> for #from_type {
                        type IntoIter = ::std::iter::Once<#ident>;
                        fn into_items(self) -> Self::IntoIter {
                            ::std::iter::once(#ident::from(self))
                        }
                    }
                }
            }
        });
        
        let expanded = quote! {
            #(#expanded_impls)*
        };
        TokenStream::from(expanded)
    } else {
        // Generic case - the from_types are specific types that should convert
        // to any instantiation of the generic struct
        let generic_params = &context.generics.params;

        let expanded_impls = from_types.iter().map(|from_type| {
            // Extract lifetimes from the from_type
            let lifetimes = extract_lifetimes(from_type);
            let unique_lifetimes: HashSet<_> = lifetimes.iter().cloned().collect();
            let lifetime_params: Vec<GenericParam> = unique_lifetimes
                .into_iter()
                .map(|lt| GenericParam::Lifetime(syn::LifetimeParam::new(lt)))
                .collect();
            
            // Combine lifetime parameters with existing generic parameters
            let all_params = if lifetime_params.is_empty() {
                quote! { #generic_params }
            } else {
                quote! { #(#lifetime_params,)* #generic_params }
            };
            
            let where_clause = combine_where_clause(
                context,
                ::std::iter::once(quote! { #ident #ty_generics: From<#from_type> }),
            );
            
            quote! {
                impl<#all_params> itemize::IntoItems<#ident #ty_generics> for #from_type
                #where_clause
                {
                    type IntoIter = ::std::iter::Once<#ident #ty_generics>;
                    fn into_items(self) -> Self::IntoIter {
                        ::std::iter::once(#ident::from(self))
                    }
                }
            }
        });

        let expanded = quote! {
            #(#expanded_impls)*
        };

        TokenStream::from(expanded)
    }
}
