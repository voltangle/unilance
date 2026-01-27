use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error, Item, LitStr, Result, Token, parse_macro_input};

struct Args {
    roles: Vec<LitStr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let roles: Punctuated<LitStr, Token![,]> = Punctuated::parse_terminated(input)?;
        if roles.is_empty() {
            return Err(Error::new(
                input.span(),
                "Expected at least one role string literal.",
            ));
        }
        Ok(Self {
            roles: roles.into_iter().collect(),
        })
    }
}

fn cfg_for_role(role_lit: &LitStr) -> Result<proc_macro2::TokenStream> {
    let role = role_lit.value();
    let ts = match role.as_str() {
        "control" => quote! {
            feature = "role_control"
        },
        "control_exclusive" => quote! {
            all(feature = "role_control", not(feature = "role_supervisor"))
        },
        "supervisor" => quote! {
            feature = "role_supervisor"
        },
        "supervisor_exclusive" => quote! {
            all(not(feature = "role_control"), feature = "role_supervisor")
        },
        "combined" => quote! {
            all(feature = "role_control", feature = "role_supervisor")
        },
        "either" => quote! {
            all(
                any(feature = "role_control", feature = "role_supervisor"),
                not(all(feature = "role_control", feature = "role_supervisor"))
            )
        },
        _ => {
            return Err(Error::new_spanned(
                role_lit,
                "Invalid role. Use: \"control\", \"supervisor\", \"either\", or \"combined\".",
            ));
        }
    };
    Ok(ts)
}

#[proc_macro_attribute]
pub fn for_role(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args);

    // Parse the annotated thing as an Item so we can re-emit it cleanly.
    let item_ast = parse_macro_input!(item as Item);

    let mut cfgs = Vec::with_capacity(args.roles.len());
    for role_lit in &args.roles {
        match cfg_for_role(role_lit) {
            Ok(cfg) => cfgs.push(cfg),
            Err(e) => return e.to_compile_error().into(),
        }
    }

    let cfg_expr = if cfgs.len() == 1 {
        // Single role => no need for any(...)
        let one = &cfgs[0];
        quote!(#one)
    } else {
        // Multiple roles => any(role1, role2, ...)
        quote!(any(#(#cfgs),*))
    };

    let expanded = quote! {
        #[cfg(#cfg_expr)]
        #item_ast
    };

    expanded.into()
}
