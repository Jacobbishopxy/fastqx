//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:52:19 Saturday
//! brief:

mod decl;
mod dr;
mod helper;
mod sql;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use dr::{fqx_cst, fqx_sql};

#[proc_macro_derive(FqxCst, attributes(fastqx))]
pub fn derive_fastqx_cst(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = fqx_cst(&input);

    // Debug use:
    // println!("{}", &stream);

    TokenStream::from(stream)
}

#[proc_macro_derive(FqxSql, attributes(fastqx))]
pub fn derive_fastqx_sql(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = fqx_sql(&input);

    // Debug use:
    // println!("{}", &stream);

    TokenStream::from(stream)
}
