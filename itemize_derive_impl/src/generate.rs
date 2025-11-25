// use proc_macro2::TokenStream;

pub mod into_items;
pub mod into_rows;
pub mod try_into_items;

// pub use into_items::IntoItemsConfig;
// use quote::quote;

// use crate::context::{CollectionType, Context};

// pub enum GenerateKind {
//     IntoItems(IntoItemsConfig),
//     TryIntoItems,
//     IntoRows,
//     TryIntoRows,
// }

// impl GenerateKind {
//     pub fn into_items(ctx: &Context<'_>) -> TokenStream {
//         let mut all: Vec<TokenStream> = vec![];
//         for collection_type in &ctx.attributes.collections {
//             match collection_type {
//                 CollectionType::Vec => all.push(IntoItemsConfig::vec(ctx).generate(ctx)),
//                 _ => (),
//             }
//         }
//         quote! { #(#all)* }
//     }
// }
