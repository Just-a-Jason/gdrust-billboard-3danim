use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemEnum, parse_macro_input};

pub fn expand_godot_enum(item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemEnum);

    let mut derive_found = false;
    for attr in &input.attrs {
        if attr.path().is_ident("derive") {
            derive_found = true;
            break;
        }
    }

    if !derive_found {
        input.attrs.push(syn::parse_quote!(
            #[derive(GodotConvert, Var, Export, Default, Copy, Clone, Debug, BilboardAnimation)]
        ));
    }

    input.attrs.push(syn::parse_quote!(
        #[godot(via = GString)]
    ));

    TokenStream::from(quote!(#input))
}
