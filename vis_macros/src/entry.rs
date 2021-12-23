use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub(crate) fn main(args: TokenStream, item: TokenStream) -> TokenStream {
    let input: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(it) => it,
        Err(e) => {
            item.clone().extend(TokenStream::from(e.into_compile_error()));
            return item
        }
    };
    TokenStream::new() 
}