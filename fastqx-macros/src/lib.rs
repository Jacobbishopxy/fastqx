//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:52:19 Saturday
//! brief:

mod dr;
mod helper;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use dr::impl_create_table;

#[proc_macro_derive(FqxTable, attributes(fastqx))]
pub fn derive_fastqx(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = impl_create_table(&input);

    // Debug use:
    // println!("{}", &stream);

    TokenStream::from(stream)
}
