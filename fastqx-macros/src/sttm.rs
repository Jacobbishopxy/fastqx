//! file: sttm.rs
//! author: Jacob Xie
//! date: 2023/09/18 15:30:24 Monday
//! brief:

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

use crate::helper::*;

// ================================================================================================
// Sqlx
// ================================================================================================

fn _extend_column_def(
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

fn _gen_column_def(f: &Field) -> TokenStream {
    let fd = f.ident.as_ref().unwrap().to_string();
    let ty = &f.ty;
    let (is_option, type_name) = get_option_type_name(ty);

    let mut res = match type_name.as_str() {
        "bool" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Boolean)
        },
        "u8" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::TinyUnsigned)
        },
        "u16" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::SmallUnsigned)
        },
        "u32" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Unsigned)
        },
        "u64" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::BigUnsigned)
        },
        "i8" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::TinyInteger)
        },
        "i16" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::SmallInteger)
        },
        "i32" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Integer)
        },
        "i64" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::BigInteger)
        },
        "f32" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Float)
        },
        "f64" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Double)
        },
        "String" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::String(None))
        },
        "Vec<u8>" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Binary(::fastqx::sea_query::BlobSize(None)))
        },
        _ => panic!("unsupported type!"),
    };

    // extension
    let (primary_key, auto_increment, unique_key) = get_col_attr(&f.attrs);
    let ext = _extend_column_def(primary_key, auto_increment, unique_key, is_option);

    res.extend(ext);

    res
}

pub(crate) fn sqlx_create_table(table_name: &str, named_fields: &NamedFields) -> TokenStream {
    let column_defs = named_fields.iter().map(_gen_column_def).collect::<Vec<_>>();

    let mut create_table_sttm = quote! {
        ::fastqx::sea_query::Table::create()
            .table(::fastqx::sea_query::Alias::new(#table_name))
            .if_not_exists()
    };
    for col_def in column_defs.iter() {
        create_table_sttm.extend(quote! {.col(&mut #col_def)});
    }

    create_table_sttm.extend(quote! {.to_owned()});

    create_table_sttm
}

pub(crate) fn sqlx_drop_table(table_name: &str) -> TokenStream {
    quote! {
        ::fastqx::sea_query::Table::drop().table(::fastqx::sea_query::Alias::new(#table_name)).to_owned()
    }
}

pub(crate) fn sqlx_insert(
    table_name: &str,
    named_fields: &NamedFields,
    struct_name: &Ident,
) -> TokenStream {
    let column_names = named_fields
        .iter()
        .map(|n| {
            let n = n.ident.as_ref().unwrap();
            quote! { #n }
        })
        .collect::<Vec<_>>();
    let column_intos = named_fields
        .iter()
        .map(|n| {
            let n = n.ident.as_ref().unwrap();
            quote! { #n.into() }
        })
        .collect::<Vec<_>>();
    let column_alias = named_fields
        .iter()
        .map(|n| {
            let n = n.ident.as_ref().unwrap().to_string();
            quote! { ::fastqx::sea_query::Alias::new(#n) }
        })
        .collect::<Vec<_>>();

    quote! {{
        let mut query = ::fastqx::sea_query::Query::insert();
        query
            .into_table(::fastqx::sea_query::Alias::new(#table_name))
            .columns([#(#column_alias),*]);

        for #struct_name {#(#column_names),*} in data.into_iter() {
            query.values([#(#column_intos),*])?;
        }

        Ok::<_, ::fastqx::anyhow::Error>(query)
    }}
}
