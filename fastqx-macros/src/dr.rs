//! file: dr.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:53:23 Saturday
//! brief:

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, DeriveInput, Field, Ident, Meta, Token};

use crate::helper::*;

fn get_col_attr(attrs: &[Attribute]) -> (bool, bool, bool) {
    let (mut primary_key, mut auto_increment, mut unique_key) = (false, false, false);

    for attr in attrs.iter() {
        // find attribute belongs to `fastqx`
        if attr.path().is_ident("fastqx") {
            let n = attr
                .parse_args_with(Punctuated::<Meta, Token!(,)>::parse_terminated)
                .unwrap()
                .iter()
                .filter_map(|a| a.path().get_ident().map(ToString::to_string))
                .collect::<Vec<_>>();

            if n.iter().any(|x| x == "primary_key") {
                primary_key = true;
            }
            if n.iter().any(|x| x == "auto_increment") {
                auto_increment = true;
            }
            if n.iter().any(|x| x == "unique_key") {
                unique_key = true;
            }

            break;
        }
    }

    (primary_key, auto_increment, unique_key)
}

fn extend_column_def(
    primary_key: bool,
    auto_increment: bool,
    unique_key: bool,
    is_option: bool,
) -> TokenStream {
    let mut res = quote!();
    if primary_key {
        res.extend(quote! {.primary_key()});
    }
    if auto_increment {
        res.extend(quote! {.auto_increment()});
    }
    if unique_key {
        res.extend(quote! {.unique_key()});
    }
    if !is_option {
        res.extend(quote! {.not_null()});
    }

    res
}

fn gen_column_def(f: &Field) -> TokenStream {
    let fd = f.ident.as_ref().unwrap().to_string();
    let ty = &f.ty;
    let (is_option, type_name) = get_option_type_name(ty);

    let mut res = match type_name.as_str() {
        "bool" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::Boolean)
        },
        "i8" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::TinyInteger)
        },
        "i16" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::SmallInteger)
        },
        "i32" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::Integer)
        },
        "i64" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::BigInteger)
        },
        "u8" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::TinyUnsigned)
        },
        "u16" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::SmallUnsigned)
        },
        "u32" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::Unsigned)
        },
        "u64" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::BigUnsigned)
        },
        "f32" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::Float)
        },
        "f64" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::Double)
        },
        "String" => quote! {
            ::fastqx_core::sea_query::ColumnDef::new_with_type(::fastqx_core::sea_query::Alias::new(#fd), ::fastqx_core::sea_query::ColumnType::String(None))
        },
        _ => panic!("unsupported type!"),
    };

    // extension
    let (primary_key, auto_increment, unique_key) = get_col_attr(&f.attrs);
    let ext = extend_column_def(primary_key, auto_increment, unique_key, is_option);

    res.extend(ext);

    res
}

fn impl_connector_statement(struct_name: &Ident, column_defs: &[TokenStream]) -> TokenStream {
    let table_name = struct_name.to_string().to_lowercase();
    let mut sttm = quote! {
        ::fastqx_core::sea_query::Table::create()
            .table(::fastqx_core::sea_query::Alias::new(#table_name))
            .if_not_exists()
    };
    for col_def in column_defs.iter() {
        sttm.extend(quote! {.col(#col_def)});
    }

    sttm.extend(quote! {.to_owned()});

    quote! {
        impl ::fastqx_core::conn::db::ConnectorStatement for #struct_name {
            fn create_table() -> ::fastqx_core::sea_query::TableCreateStatement {
                #sttm
            }

            fn drop_table() -> ::fastqx_core::sea_query::TableDropStatement {
                ::fastqx_core::sea_query::Table::drop()
                    .table(::fastqx_core::sea_query::Alias::new(#table_name)).to_owned()
            }
        }
    }
}

pub(crate) fn impl_create_table(input: &DeriveInput) -> TokenStream {
    let struct_name = input.ident.clone();
    let named_fields = named_fields(input);

    let column_defs = named_fields.iter().map(gen_column_def).collect::<Vec<_>>();

    let impl_connector_statement = impl_connector_statement(&struct_name, &column_defs);

    let expanded = quote! {
        #impl_connector_statement
    };

    expanded
}
