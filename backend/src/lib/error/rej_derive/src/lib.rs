extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn derive_rej(args: TokenStream, input: TokenStream) -> TokenStream {

    let mut iter = args.into_iter();

    let reject = match iter.next() {
        None => panic!("Error: expected text argument with error description."),
        Some(token) => match token {
            proc_macro::TokenTree::Literal(literal) => literal.to_string(),
            _ => panic!("Error: expected literal argument."),
        },
    };
    match iter.next() {
        None => panic!("Error: expected ,"),
        Some(token) => match token {
            proc_macro::TokenTree::Punct(_) => (),
            _ => panic!("Error: expected punct argument."),
        },
    };
    let code = match iter.next() {
        None => panic!("Error: expected u16 argument with error code."),
        Some(token) => match token {
            proc_macro::TokenTree::Literal(literal) => u16::from_str(&literal.to_string()).unwrap(),
            _ => panic!("Error: expected literal argument."),
        },
    };
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = ast.ident.clone();
    let expanded = quote! {
        #ast
        impl #ident {
            pub fn rej() -> Rejection {
                ErrorModel{ message: #reject.to_owned(), code: #code }.into()
            }
        }
    };
    expanded.into()
}
