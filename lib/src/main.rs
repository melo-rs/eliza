#![feature(macro_metavar_expr)]

use eliza::{patch, post, routes};

#[post("/accounts")]
fn create_account() -> &'static str {
    "create_account"
}

#[patch("/profile")]
fn update_profile() -> &'static str {
    "update_profile"
}

mod routes {
    pub mod account {
        use eliza::delete;

        #[delete("/account")]
        pub fn delete_account() -> &'static str {
            "delete_account"
        }
    }
}

fn main() {
    let router = routes![create_account, update_profile, routes::account::delete_account];

    println!("{router:?}");
}
