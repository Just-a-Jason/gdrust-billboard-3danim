use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_bilboard_animation_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &input.data {
        variants
    } else {
        panic!("BilboardAnimation can only be derived for enums");
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let lower_name = variant_name.to_string().to_lowercase();

        quote! {
            #name::#variant_name => match direction {
                Direction::Back => concat!(#lower_name, "_back"),
                Direction::Right | Direction::Left => concat!(#lower_name, "_side"),
                Direction::Front => concat!(#lower_name, "_front"),
            },
        }
    });

    let expanded = quote! {
        impl BilboardAnimation for #name {
            fn rotate(&self, direction: Direction) -> &'static str {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
