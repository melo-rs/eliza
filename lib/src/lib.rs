#![feature(macro_metavar_expr)]

#[macro_use]
pub(crate) mod syscall;
pub(crate) mod constants;
pub(crate) mod simd;
pub(crate) mod parser;
pub(crate) mod iter;

pub mod errno;
pub mod error;
pub mod fd;
pub mod io;
pub mod net;
pub mod protocol;
pub mod response;
pub mod routing;
pub mod server;

#[doc(inline)]
pub use eliza_macros::{connect, delete, get, head, options, patch, post, put, trace};

#[doc(hidden)]
pub use eliza_macros::__build_router;

#[macro_export]
macro_rules! routes {
    (
        $vis:vis static $name:ident = [
            $(
                $($route:ident)::+
            ),*
            $(,)?
        ];
    ) => {
        macro_rules! collect_routes {
            (
                [
                    [$$($$head:ident)::+]
                    $$(, [$$($$tail:ident)::+])*
                    $$(,)?
                ]
                [$$($$patterns:tt)*]
                [$$($$routes:tt)*]
            ) => {
                $$($$head)::+::metadata! {
                    $$($$head)::+,
                    collect_routes,
                    [
                        $$(
                            [$$($$tail)::+]
                        ),*
                    ],
                    [$$($$patterns)*],
                    [$$($$routes)*]
                }
            };

            (
                []
                [$$($$patterns:tt)*]
                [$$($$routes:tt)*]
            ) => {
                $crate::__build_router! {
                    [$vis]
                    $name
                    [$$($$patterns)*]
                    [$$($$routes)*]
                }
            };
        }

        collect_routes! {
            [
                $(
                    [$($route)::+]
                ),*
            ]
            []
            []
        }
    };
}
