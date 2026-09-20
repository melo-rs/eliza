mod route;
mod router;

use proc_macro::TokenStream;
use quote::format_ident;

/// Attribute to mark a handler as a GET route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("GET"))
}

/// Attribute to mark a handler as a HEAD route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn head(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("HEAD"))
}

/// Attribute to mark a handler as a POST route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("POST"))
}

/// Attribute to mark a handler as a PUT route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("PUT"))
}

/// Attribute to mark a handler as a DELETE route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("DELETE"))
}

/// Attribute to mark a handler as a CONNECT route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn connect(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("CONNECT"))
}

/// Attribute to mark a handler as an OPTIONS route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn options(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("OPTIONS"))
}

/// Attribute to mark a handler as a TRACE route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn trace(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("TRACE"))
}

/// Attribute to mark a handler as a PATCH route. This and all other route 
/// attributes can only be applied to free functions.
#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    route::route_macro(attr, item, format_ident!("PATCH"))
}

#[proc_macro]
pub fn __build_router(input: TokenStream) -> TokenStream {
    router::build_router_macro(input)
}
