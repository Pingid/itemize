use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{GenericParam, Lifetime};

use crate::context::Context;
use crate::trait_config::{TraitConfig, TraitKind};
use crate::util::{build_ordered_impl_generics, combine_where_clause};

pub fn from_types(context: &Context, config: &TraitConfig) -> TokenStream {
    if config.kind != TraitKind::Items {
        return TokenStream::new();
    }

    let impls = context
        .attributes
        .types
        .iter()
        .map(|from_type| generate_type_impl(context, config, from_type));

    quote! { #(#impls)* }
}

fn generate_type_impl(
    context: &Context,
    config: &TraitConfig,
    from_type: &syn::Type,
) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };
    let trait_path = config.trait_path();
    let trait_generics = config.trait_generics(&target);
    let method_name = config.method_name();

    // Extract lifetimes from the from_type
    let lifetime_params: Vec<TokenStream> = extract_lifetimes(from_type)
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|lt| {
            let param = GenericParam::Lifetime(syn::LifetimeParam::new(lt));
            quote! { #param }
        })
        .collect();

    let impl_generics = build_ordered_impl_generics(context, config, &lifetime_params);
    let item_type = config.iterator_item_type(&target);
    let into_iter_type = quote! { ::std::iter::Once<#item_type> };

    let (conversion, where_clause) = if config.is_try() {
        let error_type = config.error_type_tokens();
        let conversion = quote! {
            #ident::try_from(self).map_err(|e| Into::<#error_type>::into(e))
        };
        let where_clause = combine_where_clause(
            context,
            [
                quote! { #target: ::std::convert::TryFrom<#from_type> },
                quote! { <#target as ::std::convert::TryFrom<#from_type>>::Error: Into<#error_type> },
            ],
        );
        (conversion, where_clause)
    } else {
        let conversion = config.wrap_conversion(quote! { #ident::from(self) });
        let where_clause = combine_where_clause(context, [quote! { #target: From<#from_type> }]);
        (conversion, where_clause)
    };

    quote! {
        impl #impl_generics #trait_path<#trait_generics> for #from_type
        #where_clause
        {
            type IntoIter = #into_iter_type;
            fn #method_name(self) -> Self::IntoIter {
                ::std::iter::once(#conversion)
            }
        }
    }
}

fn extract_lifetimes(ty: &syn::Type) -> Vec<Lifetime> {
    let mut lifetimes = Vec::new();

    match ty {
        syn::Type::Reference(ref_ty) => {
            if let Some(lifetime) = &ref_ty.lifetime {
                lifetimes.push(lifetime.clone());
            }
            lifetimes.extend(extract_lifetimes(&ref_ty.elem));
        }
        syn::Type::Path(path_ty) => {
            if let Some(qself) = &path_ty.qself {
                lifetimes.extend(extract_lifetimes(&qself.ty));
            }
            for segment in &path_ty.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        match arg {
                            syn::GenericArgument::Lifetime(lt) => lifetimes.push(lt.clone()),
                            syn::GenericArgument::Type(ty) => {
                                lifetimes.extend(extract_lifetimes(ty))
                            }
                            _ => {}
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
