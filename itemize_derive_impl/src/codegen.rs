use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::context::Context;
use crate::trait_config::{AssociatedTypes, TraitConfig, TraitKind};
use crate::util::build_ordered_impl_generics;

pub fn generate_derive(input: TokenStream, config: TraitConfig) -> TokenStream {
    let ast = match syn::parse2::<DeriveInput>(input) {
        Ok(ast) => ast,
        Err(e) => return e.to_compile_error(),
    };

    let context = match Context::try_new(&ast) {
        Ok(ctx) => ctx,
        Err(e) => return e.to_compile_error(),
    };

    let config = config.with_error_type(context.attributes.error_type.clone());
    generate_trait_implementation(&context, config)
}

fn generate_trait_implementation(context: &Context, config: TraitConfig) -> TokenStream {
    let base_impl = generate_base_impl(context, &config);
    let type_impls = crate::unified::types::from_types(context, &config);
    let tuple_impls = crate::unified::tuples::from_tuples(context, &config);
    let collection_impls = crate::unified::collections::from_collections(context, &config);

    quote! {
        #base_impl
        #type_impls
        #tuple_impls
        #collection_impls
    }
}

fn generate_base_impl(context: &Context, config: &TraitConfig) -> TokenStream {
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let target = quote! { #ident #ty_generics };

    let trait_path = config.trait_path();
    let trait_generics = config.trait_generics(&target);
    let method_name = config.method_name();
    let impl_generics = build_ordered_impl_generics(context, config, &[]);
    let where_clause = &context.where_clause;

    match config.kind {
        TraitKind::Items => {
            let item_type = config.iterator_item_type(&target);
            let conversion = config.wrap_conversion(quote! { self });
            let associated_types = config.associated_types(AssociatedTypes::for_items(
                quote! { ::std::iter::Once<#item_type> },
            ));

            quote! {
                impl #impl_generics #trait_path<#trait_generics> for #target #where_clause {
                    #associated_types
                    fn #method_name(self) -> Self::IntoIter {
                        ::std::iter::once(#conversion)
                    }
                }
            }
        }
        TraitKind::Rows => {
            let row_item_type = config.iterator_item_type(&target);
            let conversion = config.wrap_conversion(quote! { self });
            let associated_types = config.associated_types(AssociatedTypes::for_rows(
                quote! { std::iter::Once<#row_item_type> },
                quote! { std::iter::Once<Self::RowIter> },
            ));

            quote! {
                impl #impl_generics #trait_path<#trait_generics> for #target #where_clause {
                    #associated_types
                    fn #method_name(self) -> Self::Rows {
                        std::iter::once(std::iter::once(#conversion))
                    }
                }
            }
        }
    }
}
