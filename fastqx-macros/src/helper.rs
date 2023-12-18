//! file: helper.rs
//! author: Jacob Xie
//! date: 2023/09/10 00:29:28 Sunday
//! brief:

use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Data, DeriveInput, Field, Fields, Meta, Token, Type};

pub(crate) type NamedFields = Punctuated<Field, Comma>;

pub(crate) fn named_fields(ast: &DeriveInput) -> NamedFields {
    match &ast.data {
        Data::Struct(s) => {
            if let Fields::Named(ref named_fields) = s.fields {
                named_fields.named.clone()
            } else {
                unimplemented!("derive(Builder) only supports named fields")
            }
        }
        _ => unimplemented!("fx only supports Struct and is not implemented for Enum/Union"),
    }
}

fn path_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(tp) => {
            let path = &tp.path;
            tp.qself.is_none()
                && path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments.iter().next().unwrap().ident == "Option"
        }
        _ => panic!("type mismatch"),
    }
}

fn get_option_type(ty: &Type) -> (bool, String) {
    let is_option = path_is_option(ty);

    if let Type::Path(tp) = ty {
        let path = &tp.path;
        if is_option {
            match &path.segments.first().unwrap().arguments {
                syn::PathArguments::AngleBracketed(ab) => {
                    let ga = ab.args.first().unwrap();
                    match ga {
                        syn::GenericArgument::Type(Type::Path(t)) => {
                            return (
                                true,
                                t.path
                                    .segments
                                    .to_token_stream()
                                    .to_string()
                                    .replace(" ", ""),
                            );
                        }
                        _ => panic!("[get_option_type] type mismatch"),
                    }
                }
                _ => panic!("[get_option_type] type mismatch"),
            }
        } else {
            return (
                false,
                path.segments.to_token_stream().to_string().replace(" ", ""),
            );
        }
    }

    panic!("type mismatch")
}

pub(crate) fn get_option_type_name(ty: &Type) -> (bool, String) {
    let (is_option, ident) = get_option_type(ty);
    (is_option, ident)
}

pub(crate) fn get_col_attr(attrs: &[Attribute]) -> (bool, bool, bool) {
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
