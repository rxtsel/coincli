mod utils;

use crate::utils::ascii::start_ascii;
use crate::utils::currencies::{format_cop_price, format_usd_price};
use crate::utils::data::get_coin_data;
use std::io;

fn main() {
    let mut coin_name = String::new();

    start_ascii();

    println!("Enter crypto name: ");

    io::stdin()
        .read_line(&mut coin_name)
        .expect("Failed to read line");

    // Shadowing: trim and lowercase convert
    let coin_name = coin_name.trim().to_lowercase();

    // Error management
    match get_coin_data(&coin_name) {
        Ok(data) => match data.get(&coin_name) {
            Some(price) => {
                println!(
                    "Current {} price: {}, {}",
                    coin_name,
                    format_usd_price(price.usd),
                    format_cop_price(price.cop)
                );
            }
            None => {
                println!("Crypto not found");
            }
        },
        Err(error) => {
            println!("Request failed: {error}");
        }
    }
}
