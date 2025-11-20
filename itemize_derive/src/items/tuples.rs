use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::context::Context;

pub fn from_tuples(context: &Context) -> TokenStream {
    let Some(max) = context.attributes.from_tuples else {
        return TokenStream::new();
    };

    let into_items_trait = &context.trait_ident;
    let ident = &context.ident;
    let ty_generics = &context.ty_generics;
    let mut expanded = TokenStream::new();

    for n in 1..=max {
        // Types: A0, A1, ..., A{n-1}
        let type_params: Vec<_> = (0..n).map(|i| format_ident!("A{}", i)).collect();

        // Value bindings: a0, a1, ..., a{n-1}
        let vars: Vec<_> = (0..n).map(|i| format_ident!("a{}", i)).collect();

        let len = n; // array size

        expanded.extend(quote! {
            // impl<A0> itemize_2::IntoItems<MySimpleType> for (A0,)
            // where
            //     MySimpleType: From<A0>,
            // {
            //     type IntoIter = std::array::IntoIter<MySimpleType, 1usize>;
            //     fn into_items(self) -> Self::IntoIter {
            //         let (a0,) = self;
            //         <[MySimpleType; 1]>::into_iter([MySimpleType::from(a0)])
            //     }
            // }
            
            
            impl<#(#type_params),*> itemize_2::IntoItems<#ident #ty_generics> for (#(#type_params,)*)
            where
                #(#ident #ty_generics: From<#type_params>,)*
            {
                type IntoIter = std::array::IntoIter<#ident #ty_generics, #len>;

                fn into_items(self) -> Self::IntoIter {
                    let (#(#vars,)*) = self;
                    <[#ident #ty_generics; #len]>::into_iter([#(#ident::from(#vars)),*])
                }
            }
        });
    }

    expanded
}
