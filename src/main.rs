mod consts;

mod handlers;
mod rest;
mod utils;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/error_codes.rs"));
}

use generated::{ErrorCodes, ErrorTypes};

fn main() {
    println!("bybit")
}
