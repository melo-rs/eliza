use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, LitStr, parse_macro_input};

pub(crate) fn route_macro(attr: TokenStream, item: TokenStream, method: Ident) -> TokenStream {
    let path = parse_macro_input!(attr as LitStr);
    let function = parse_macro_input!(item as ItemFn);

    let function_visibility = &function.vis;
    let function_name = &function.sig.ident;

    quote! {
        #function

        #[doc(hidden)]
        #function_visibility mod #function_name {
            #[inline]
            pub fn handler() -> ::eliza::response::Response {
                ::eliza::response::IntoResponse::into_response(super::#function_name())
            }

            macro_rules! metadata {
                (
                    $handler:path,
                    $callback:ident,
                    [$($rest:tt)*],
                    [$($patterns:tt)*],
                    [$($routes:tt)*]
                ) => {
                    $callback! {
                        [$($rest)*]
                        [
                            $($patterns)*
                            #path,
                        ]
                        [
                            $($routes)*
                            (#method, $handler::handler),
                        ]
                    }
                };
            }

            pub(crate) use metadata;
        }
    }
    .into()
}
