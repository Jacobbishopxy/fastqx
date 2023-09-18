//! file: dr.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:53:23 Saturday
//! brief:

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Attribute, DeriveInput, Field, Ident, Meta, Token};

use crate::helper::*;

// ================================================================================================
// ConnectorStatement
// sea_query table statements
// ================================================================================================

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
    let ext = extend_column_def(primary_key, auto_increment, unique_key, is_option);

    res.extend(ext);

    res
}

fn impl_connector_statement(struct_name: &Ident, named_fields: &NamedFields) -> TokenStream {
    let column_defs = named_fields.iter().map(gen_column_def).collect::<Vec<_>>();
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

    let table_name = struct_name.to_string().to_lowercase();
    let mut create_table_sttm = quote! {
        ::fastqx::sea_query::Table::create()
            .table(::fastqx::sea_query::Alias::new(#table_name))
            .if_not_exists()
    };
    for col_def in column_defs.iter() {
        create_table_sttm.extend(quote! {.col(&mut #col_def)});
    }

    create_table_sttm.extend(quote! {.to_owned()});

    quote! {
        impl ::fastqx::sql::conn::ConnectorStatement for #struct_name {
            fn create_table() -> ::fastqx::sea_query::TableCreateStatement {
                #create_table_sttm
            }

            fn drop_table() -> ::fastqx::sea_query::TableDropStatement {
                ::fastqx::sea_query::Table::drop()
                    .table(::fastqx::sea_query::Alias::new(#table_name)).to_owned()
            }

            fn insert(data: Vec<Self>) -> ::fastqx::anyhow::Result<::fastqx::sea_query::InsertStatement> {
                let mut query = ::fastqx::sea_query::Query::insert();
                query
                    .into_table(::fastqx::sea_query::Alias::new(#table_name))
                    .columns([#(#column_alias),*]);

                for #struct_name {#(#column_names),*} in data.into_iter() {
                    query.values([#(#column_intos),*])?;
                }

                Ok(query)
            }
        }
    }
}

// ================================================================================================
// sqlx FrowRow
// ================================================================================================

fn gen_sqlx_column_try(f: &Field) -> TokenStream {
    let fd = f.ident.as_ref().unwrap();
    let fd_str = fd.to_string();

    quote! {
        #fd: row.try_get(#fd_str)?
    }
}

fn gen_tiberius_column_try(f: &Field) -> TokenStream {
    let fd = f.ident.as_ref().unwrap();
    let fd_str = fd.to_string();

    quote! {
        #fd: ::fastqx::sql::TryGetFromRow::try_get(row, #fd_str)?
    }
}

fn impl_from_row(struct_name: &Ident, named_fields: &NamedFields) -> TokenStream {
    let sqlx_column_try = named_fields
        .iter()
        .map(gen_sqlx_column_try)
        .collect::<Vec<_>>();
    let tiberius_column_try = named_fields
        .iter()
        .map(gen_tiberius_column_try)
        .collect::<Vec<_>>();

    quote! {
        use ::fastqx::sqlx::Row;
        use ::fastqx::sql::TryGetFromRow;

        impl ::fastqx::sqlx::FromRow<'_, ::fastqx::sqlx::mysql::MySqlRow> for #struct_name {
            fn from_row(row: &::fastqx::sqlx::mysql::MySqlRow) -> ::fastqx::sqlx::Result<Self> {
                Ok(Self {
                    #(#sqlx_column_try),*
                })
            }
        }


        impl ::fastqx::sqlx::FromRow<'_, ::fastqx::sqlx::postgres::PgRow> for #struct_name {
            fn from_row(row: &::fastqx::sqlx::postgres::PgRow) -> ::fastqx::sqlx::Result<Self> {
                Ok(Self {
                    #(#sqlx_column_try),*
                })
            }
        }


        impl ::fastqx::sqlx::FromRow<'_, ::fastqx::sqlx::sqlite::SqliteRow> for #struct_name {
            fn from_row(row: &::fastqx::sqlx::sqlite::SqliteRow) -> ::fastqx::sqlx::Result<Self> {
                Ok(Self {
                    #(#sqlx_column_try),*
                })
            }
        }

        impl<'r> ::fastqx::sql::FromTiberiusRow<'r> for #struct_name {
            fn from_row(row: &'r ::fastqx::tiberius::Row) -> ::fastqx::anyhow::Result<Self> {
                Ok(Self {
                    #(#tiberius_column_try),*
                })
            }
        }
    }
}

// ================================================================================================
// FqxSchema
// ================================================================================================

pub(crate) fn impl_fqx_schema(input: &DeriveInput) -> TokenStream {
    let struct_name = input.ident.clone();
    let named_fields = named_fields(input);

    // sqlx::FromRow
    let impl_from_row = impl_from_row(&struct_name, &named_fields);

    // sea_query Table statements
    let impl_connector_statement = impl_connector_statement(&struct_name, &named_fields);

    let expanded = quote! {
        #impl_from_row

        #impl_connector_statement
    };

    expanded
}
