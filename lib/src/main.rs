#![feature(macro_metavar_expr)]

use eliza::{post, routes};

#[post("/accounts")]
fn create_account() -> &'static str {
    "create_account"
}

pub mod account {
    use eliza::delete;

    #[delete("/account")]
    pub fn delete_account() -> &'static str {
        "delete_account"
    }
}

routes! {
    static ROUTER = [
        create_account,
        account::delete_account,
    ];
}

fn main() {}
