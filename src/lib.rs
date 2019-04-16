#![allow(unused_variables)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn range(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("--- attr ---\n{:?}\n", attr);
    println!("--- item ---\n{:?}\n", item);
    //let pattr = syn::parse_macro_input!(attr as syn::ExprRange);
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;

    // TODO: We're gonna need hygiene SOMEWHERE in here, right?
    let result = quote! {
        struct #name(i32);
        impl #name {
            fn new(value: i32) -> #name { #name(value) }
            const LOWER: i32 = 99;
            const UPPER: i32 = 999;
        }
    };
    let result: TokenStream = result.into();
    println!("--- result ---\n{:?}\n", result);
    result
}


#[cfg(test)]
mod tests {
}
