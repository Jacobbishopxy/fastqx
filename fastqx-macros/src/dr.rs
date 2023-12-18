//! file: dr.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:53:23 Saturday
//! brief:

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident};

use crate::decl::impl_fqx_cst;
use crate::helper::*;
use crate::sttm::*;

// ================================================================================================
// ConnectorStatement
// sea_query table statements
// ================================================================================================

fn impl_connector_statement(struct_name: &Ident, named_fields: &NamedFields) -> TokenStream {
    let table_name = struct_name.to_string().to_lowercase();

    let sqlx_ct = sqlx_create_table(&table_name, named_fields);
    let sqlx_dt = sqlx_drop_table(&table_name);
    let sqlx_is = sqlx_insert(&table_name, named_fields, struct_name);

    let tiberius_ct = tiberius_create_table(&table_name, named_fields);
    let tiberius_dt = tiberius_drop_table(&table_name);
    let tiberius_is = tiberius_insert(&table_name, named_fields, struct_name);

    quote! {
        impl ::fastqx::sources::sql::ConnectorStatement for #struct_name {
            fn create_table(driver: &::fastqx::sources::sql::Driver) -> ::fastqx::anyhow::Result<String> {
                let res = match driver {
                    ::fastqx::sources::sql::Driver::MYSQL => #sqlx_ct.to_string(::fastqx::sea_query::MysqlQueryBuilder),
                    ::fastqx::sources::sql::Driver::POSTGRES => #sqlx_ct.to_string(::fastqx::sea_query::PostgresQueryBuilder),
                    ::fastqx::sources::sql::Driver::MSSQL => #tiberius_ct,
                    ::fastqx::sources::sql::Driver::SQLITE => #sqlx_ct.to_string(::fastqx::sea_query::SqliteQueryBuilder),
                };
                Ok(res)
            }

            fn drop_table(driver: &::fastqx::sources::sql::Driver) -> ::fastqx::anyhow::Result<String> {
                let res = match driver {
                    ::fastqx::sources::sql::Driver::MYSQL => #sqlx_dt.to_string(::fastqx::sea_query::MysqlQueryBuilder),
                    ::fastqx::sources::sql::Driver::POSTGRES => #sqlx_dt.to_string(::fastqx::sea_query::PostgresQueryBuilder),
                    ::fastqx::sources::sql::Driver::MSSQL => #tiberius_dt,
                    ::fastqx::sources::sql::Driver::SQLITE => #sqlx_dt.to_string(::fastqx::sea_query::SqliteQueryBuilder),
                };
                Ok(res)
            }

            fn insert(driver: &::fastqx::sources::sql::Driver, data: Vec<Self>) -> ::fastqx::anyhow::Result<String> {
                let res = match driver {
                    ::fastqx::sources::sql::Driver::MYSQL => #sqlx_is?.to_string(::fastqx::sea_query::MysqlQueryBuilder),
                    ::fastqx::sources::sql::Driver::POSTGRES => #sqlx_is?.to_string(::fastqx::sea_query::PostgresQueryBuilder),
                    ::fastqx::sources::sql::Driver::MSSQL => #tiberius_is,
                    ::fastqx::sources::sql::Driver::SQLITE => #sqlx_is?.to_string(::fastqx::sea_query::SqliteQueryBuilder),
                };
                Ok(res)
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
        #fd: ::fastqx::sources::sql::TryGetFromRow::try_get(row, #fd_str)?
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
        use ::fastqx::sources::sql::TryGetFromRow;

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

        impl<'r> ::fastqx::sources::sql::FromTiberiusRow<'r> for #struct_name {
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

    // fastqx::FqxCst
    let impl_fqx_cst = impl_fqx_cst(&struct_name, &named_fields);

    let expanded = quote! {
        #impl_from_row

        #impl_connector_statement

        #impl_fqx_cst
    };

    expanded
}
