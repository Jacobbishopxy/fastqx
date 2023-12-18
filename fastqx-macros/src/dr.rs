//! file: dr.rs
//! author: Jacob Xie
//! date: 2023/09/09 23:53:23 Saturday
//! brief:

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::decl::impl_fqx_cst;
use crate::helper::*;
use crate::sql::*;

// ================================================================================================
// FqxSchema
// ================================================================================================

pub(crate) fn fqx_cst(input: &DeriveInput) -> TokenStream {
    let struct_name = input.ident.clone();
    let named_fields = named_fields(input);

    // fastqx::FqxCst
    let impl_fqx_cst = impl_fqx_cst(&struct_name, &named_fields);

    let expanded = quote! {
        #impl_fqx_cst
    };

    expanded
}

pub(crate) fn fqx_sql(input: &DeriveInput) -> TokenStream {
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
