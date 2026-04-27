pub mod data {
    use reqwest::{Result, blocking::Client};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    pub struct Price {
        pub usd: f64,
        pub cop: f64,
    }

    type CoinDataResponse = HashMap<String, Price>;

    pub fn get_coin_data(coin_name: &str) -> Result<CoinDataResponse> {
        Client::new()
        .get(format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={coin_name}&vs_currencies=usd,cop"
        ))
        .header("User-Agent", "coincli/0.1 (rust)")
        .send()?
        .json::<CoinDataResponse>()
    }
}

pub mod currencies {
    use num_format::{Locale, ToFormattedString};

    pub fn format_usd_price(amount: f64) -> String {
        format_currency_price(amount, "USD", Locale::en, '.', ',')
    }

    pub fn format_cop_price(amount: f64) -> String {
        format_currency_price(amount, "COP", Locale::es, ',', '.')
    }

    fn format_currency_price(
        amount: f64,
        currency_code: &str,
        locale: Locale,
        decimal_separator: char,
        thousands_separator: char,
    ) -> String {
        let amount_in_cents = (amount * 100.0).round() as i64;

        let whole_units = amount_in_cents / 100;
        let cents = amount_in_cents.abs() % 100;

        let formatted_whole_units = whole_units
            .to_formatted_string(&locale)
            .replace([',', '.'], &thousands_separator.to_string());

        format!("${formatted_whole_units}{decimal_separator}{cents:02} {currency_code}")
    }
}

pub mod ascii {
    pub fn start_ascii() {
        println!(
            "
░█▀▀░█▀█░▀█▀░█▀█░█▀▀░█░░░▀█▀
░█░░░█░█░░█░░█░█░█░░░█░░░░█░
░▀▀▀░▀▀▀░▀▀▀░▀░▀░▀▀▀░▀▀▀░▀▀▀
by @rxtsel.
        "
        );
    }
}
