extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse::Parser, parse_macro_input};
use quote::quote;

#[proc_macro_attribute]
pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {           
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(quote! { pub rejection: String }).unwrap());
                }   
                _ => {
                    ()
                }
            }              
            
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}

//#[proc_macro_derive(ToRejection)]
#[proc_macro_attribute]
pub fn derive_rej(args: TokenStream, input: TokenStream) -> TokenStream {
    let linput = add_field(TokenStream::default(), input);

    let reject = match args.into_iter().next(){
        None => panic!("Error: expected text argument with error description."),
        Some(token) => match token{
            proc_macro::TokenTree::Literal(literal) => {
                literal.to_string()
            }
            _ => panic!("Error: expected literal argument.")
        }
    };
    let ast = parse_macro_input!(linput as DeriveInput);
    let ident = ast.ident.clone();
    let expanded = quote! {
        #ast
        impl Reject for #ident {}
        impl #ident {
            pub fn rej() -> Rejection {
                reject::custom(#ident{ rejection: #reject.to_owned() })
            }
        }
    };
    expanded.into()
}