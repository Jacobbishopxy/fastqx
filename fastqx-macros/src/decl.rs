//! file: decl.rs
//! author: Jacob Xie
//! date: 2023/12/17 22:27:15 Sunday
//! brief:

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

use crate::helper::*;

fn _gen_fqx_cols_and_types(f: &Field) -> (TokenStream, TokenStream) {
    let fd = f.ident.as_ref().unwrap().to_string();
    let ty = &f.ty;
    let (_, type_name) = get_option_type_name(ty);

    let typ = match type_name.as_str() {
        "bool" => quote! { ::fastqx::adt::val::value::FqxValueType::Bool },
        "u8" => quote! { ::fastqx::adt::val::value::FqxValueType::U8 },
        "u16" => quote! { ::fastqx::adt::val::value::FqxValueType::U16 },
        "u32" => quote! { ::fastqx::adt::val::value::FqxValueType::U32 },
        "u64" => quote! { ::fastqx::adt::val::value::FqxValueType::U64 },
        "i8" => quote! { ::fastqx::adt::val::value::FqxValueType::I8 },
        "i16" => quote! { ::fastqx::adt::val::value::FqxValueType::I16 },
        "i32" => quote! { ::fastqx::adt::val::value::FqxValueType::I32 },
        "i64" => quote! { ::fastqx::adt::val::value::FqxValueType::I64 },
        "f32" => quote! { ::fastqx::adt::val::value::FqxValueType::F32 },
        "f64" => quote! { ::fastqx::adt::val::value::FqxValueType::F64 },
        "String" => quote! { ::fastqx::adt::val::value::FqxValueType::String },
        "Vec<u8>" => quote! { ::fastqx::adt::val::value::FqxValueType::Blob },
        "DateTime<Local>" => quote! { ::fastqx::adt::val::value::FqxValueType::Timestamp },
        "NaiveDateTime" => quote! { ::fastqx::adt::val::value::FqxValueType::DateTime },
        "NaiveDate" => quote! { ::fastqx::adt::val::value::FqxValueType::Date },
        "NaiveTime" => quote! { ::fastqx::adt::val::value::FqxValueType::Time },
        a => panic!("unsupported type: {a}!"),
    };

    let col = quote! {#fd};

    (col, typ)
}

pub(crate) fn impl_fqx_cst(struct_name: &Ident, named_fields: &NamedFields) -> TokenStream {
    let (cols, types): (Vec<_>, Vec<_>) = named_fields.iter().map(_gen_fqx_cols_and_types).unzip();

    quote! {
        use ::fastqx::adt::ab::d::FqxCst;

        impl ::fastqx::adt::ab::d::FqxCst for #struct_name {
            fn new_empty() -> ::fastqx::adt::dat::data::FqxData {
                ::fastqx::adt::dat::data::FqxData::new_empty(
                    vec![#(#cols),*],
                    vec![#(#types),*],
                ).unwrap()
            }
        }
    }
}
