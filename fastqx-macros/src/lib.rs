//! file: lib.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:52:19 Saturday
//! brief:

mod dr;
mod helper;
mod sttm;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use dr::impl_fqx_schema;

#[proc_macro_derive(FqxSchema, attributes(fastqx))]
pub fn derive_fastqx(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let stream = impl_fqx_schema(&input);

    // Debug use:
    // println!("{}", &stream);

    TokenStream::from(stream)
}
