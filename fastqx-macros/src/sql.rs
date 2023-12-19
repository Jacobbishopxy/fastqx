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

fn _extend_sqlx_column(
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

fn _gen_sqlx_column(f: &Field) -> TokenStream {
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
        "DateTime<Local>" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Timestamp)
        },
        "NaiveDateTime" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::DateTime)
        },
        "NaiveDate" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Date)
        },
        "NaiveTime" => quote! {
            ::fastqx::sea_query::ColumnDef::new_with_type(::fastqx::sea_query::Alias::new(#fd), ::fastqx::sea_query::ColumnType::Time)
        },
        a => panic!("[sqlx_column] unsupported type: {a}!"),
    };

    // extension
    let (primary_key, auto_increment, unique_key) = get_col_attr(&f.attrs);
    let ext = _extend_sqlx_column(primary_key, auto_increment, unique_key, is_option);

    res.extend(ext);

    res
}

fn sqlx_create_table(table_name: &str, named_fields: &NamedFields) -> TokenStream {
    let column_defs = named_fields
        .iter()
        .map(_gen_sqlx_column)
        .collect::<Vec<_>>();

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

fn sqlx_drop_table(table_name: &str) -> TokenStream {
    quote! {
        ::fastqx::sea_query::Table::drop().table(::fastqx::sea_query::Alias::new(#table_name)).to_owned()
    }
}

fn sqlx_insert(table_name: &str, named_fields: &NamedFields, struct_name: &Ident) -> TokenStream {
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

// ================================================================================================
// Tiberius
// ================================================================================================

fn _extend_tiberius_column(
    primary_key: bool,
    auto_increment: bool,
    unique_key: bool,
    is_option: bool,
) -> String {
    let mut ext = String::new();

    if auto_increment {
        ext.push_str(" IDENTITY ");
    }

    if primary_key {
        ext.push_str(" PRIMARY KEY ");
    }

    if unique_key {
        ext.push_str(" UNIQUE ");
    }

    if !is_option {
        ext.push_str(" NOT NULL ");
    }

    ext
}

fn _gen_tiberius_column(f: &Field) -> String {
    let fd = f.ident.as_ref().unwrap().to_string();
    let ty = &f.ty;
    let (is_option, type_name) = get_option_type_name(ty);

    let mut res = match type_name.as_str() {
        "bool" => format!("{} {}", fd, "BIT"),
        "u8" => format!("{} {}", fd, "TINYINT"),
        "u16" => format!("{} {}", fd, "SMALLINT"),
        "u32" => format!("{} {}", fd, "INT"),
        "u64" => format!("{} {}", fd, "BIGINT"),
        "i8" => format!("{} {}", fd, "TINYINT"),
        "i16" => format!("{} {}", fd, "SMALLINT"),
        "i32" => format!("{} {}", fd, "INT"),
        "i64" => format!("{} {}", fd, "BIGINT"),
        "f32" => format!("{} {}", fd, "FLOAT(24)"),
        "f64" => format!("{} {}", fd, "FLOAT(53)"),
        "String" => format!("{} {}", fd, "VARCHAR(100)"),
        "Vec<u8>" => format!("{} {}", fd, "BINARY"),
        "DateTime<Local>" => format!("{} {}", fd, "DATETIMEOFFSET(7)"),
        "NaiveDateTime" => format!("{} {}", fd, "DATETIME"),
        "NaiveDate" => format!("{} {}", fd, "DATE"),
        "NaiveTime" => format!("{} {}", fd, "TIME(7)"),
        a => panic!("[tiberius_column] unsupported type: {a}!"),
    };

    // extension
    let (primary_key, auto_increment, unique_key) = get_col_attr(&f.attrs);
    let ext = _extend_tiberius_column(primary_key, auto_increment, unique_key, is_option);

    res.push_str(&ext);

    res
}

fn tiberius_create_table(table_name: &str, named_fields: &NamedFields) -> TokenStream {
    let mut res = format!(
        "IF OBJECT_ID(N'{}', N'U') IS NULL CREATE TABLE {} ",
        table_name, table_name
    );

    let cols = named_fields
        .iter()
        .map(_gen_tiberius_column)
        .collect::<Vec<_>>()
        .join(",");
    res.push('(');
    res.push_str(&cols);
    res.push_str(");");

    quote! {
        #res.to_string()
    }
}

fn tiberius_drop_table(table_name: &str) -> TokenStream {
    quote! {
        format!("DROP TABLE IF EXISTS {};", #table_name)
    }
}

fn tiberius_insert(
    table_name: &str,
    named_fields: &NamedFields,
    struct_name: &Ident,
) -> TokenStream {
    let cols = named_fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",");
    let column_names = named_fields
        .iter()
        .map(|n| {
            let n = n.ident.as_ref().unwrap();
            quote! { #n }
        })
        .collect::<Vec<_>>();
    let column_intos = named_fields
        .iter()
        .map(|f| {
            let t = &f.ty;
            let n = f.ident.as_ref().unwrap();
            let (_, type_name) = get_option_type_name(t);
            match type_name.as_str() {
                "bool" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "u8" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "u16" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "u32" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "u64" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "i8" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "i16" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "i32" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "i64" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "f32" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "f64" => quote! {  ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "String" => quote! { ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "Vec<u8>" => quote! { ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "DateTime<Local>" => quote! { ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "NaiveDateTime" => quote! { ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "NaiveDate" => quote! { ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                "NaiveTime" => quote! { ::fastqx::sources::sql::ToSqlString::to_sql(#n) },
                a => panic!("[tiberius_insert] unsupported type: {a}!"),
            }
        })
        .collect::<Vec<_>>();

    quote! {{
        let mut res = format!("SET IDENTITY_INSERT {} ON; INSERT INTO {}", #table_name, #table_name);

        res.push('(');
        res.push_str(&#cols);
        res.push_str(") VALUES ");

        let mut vals = vec![];
        for #struct_name {#(#column_names),*} in data.into_iter() {
            let v = vec![#(#column_intos),*].join(",");
            vals.push(format!("({})", v));
        }

        res.push_str(&vals.join(", "));
        res.push(';');
        res.push_str(&format!(" SET IDENTITY_INSERT {} OFF", #table_name));

        res
    }}
}

// ================================================================================================
// ConnectorStatement
// sea_query table statements
// ================================================================================================

pub(crate) fn impl_connector_statement(
    struct_name: &Ident,
    named_fields: &NamedFields,
) -> TokenStream {
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

pub(crate) fn gen_sqlx_column_try(f: &Field) -> TokenStream {
    let fd = f.ident.as_ref().unwrap();
    let fd_str = fd.to_string();

    quote! {
        #fd: row.try_get(#fd_str)?
    }
}

pub(crate) fn gen_tiberius_column_try(f: &Field) -> TokenStream {
    let fd = f.ident.as_ref().unwrap();
    let fd_str = fd.to_string();

    quote! {
        #fd: ::fastqx::sources::sql::TryGetFromTiberiusRow::try_get(&row, #fd_str)?
    }
}

pub(crate) fn impl_from_row(struct_name: &Ident, named_fields: &NamedFields) -> TokenStream {
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

        impl ::fastqx::sources::sql::FromSqlxRow<::fastqx::sqlx::mysql::MySqlRow> for #struct_name {
            fn from_row(row: ::fastqx::sqlx::mysql::MySqlRow) -> ::fastqx::sqlx::Result<Self> {
                Ok(Self {
                    #(#sqlx_column_try),*
                })
            }
        }


        impl ::fastqx::sources::sql::FromSqlxRow<::fastqx::sqlx::postgres::PgRow> for #struct_name {
            fn from_row(row: ::fastqx::sqlx::postgres::PgRow) -> ::fastqx::sqlx::Result<Self> {
                Ok(Self {
                    #(#sqlx_column_try),*
                })
            }
        }


        impl ::fastqx::sources::sql::FromSqlxRow<::fastqx::sqlx::sqlite::SqliteRow> for #struct_name {
            fn from_row(row: ::fastqx::sqlx::sqlite::SqliteRow) -> ::fastqx::sqlx::Result<Self> {
                Ok(Self {
                    #(#sqlx_column_try),*
                })
            }
        }

        impl ::fastqx::sources::sql::FromTiberiusRow for #struct_name {
            fn from_row(row: ::fastqx::tiberius::Row) -> ::fastqx::anyhow::Result<Self> {
                Ok(Self {
                    #(#tiberius_column_try),*
                })
            }
        }
    }
}
