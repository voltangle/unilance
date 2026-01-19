use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitStr, Token};

struct Args {
    role: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let role: LitStr = input.parse()?;
        // Allow optional trailing comma: #[for_role("control",)]
        let _ = input.parse::<Option<Token![,]>>()?;
        Ok(Self { role })
    }
}

#[proc_macro_attribute]
pub fn for_role(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args);
    let role = args.role.value();

    let cfg_expr = match role.as_str() {
        "control" => quote! {
            all(feature = "role_control", not(feature = "role_supervisor"))
        },
        "supervisor" => quote! {
            all(not(feature = "role_control"), feature = "role_supervisor")
        },
        "combined" => quote! {
            all(feature = "role_control", feature = "role_supervisor")
        },
        _ => {
            return syn::Error::new_spanned(
                args.role,
                "Invalid role. Use: \"control\", \"supervisor\", or \"combined\".",
            )
            .to_compile_error()
            .into();
        }
    };

    let item = proc_macro2::TokenStream::from(item);

    // Prepend a #[cfg(...)] attribute to the item.
    let expanded = quote! {
        #cfg_expr
        #item
    };

    expanded.into()
}

