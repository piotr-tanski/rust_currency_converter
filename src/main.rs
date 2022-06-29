mod currency_source;
mod converter;
mod currency;

use crate::currency_source::NBP;
use crate::converter::convert;
use crate::currency::{Currency, PLN};

fn main() {
    let mut source = NBP::new();
    let my_money = PLN::new(100);
    let eur = convert(&my_money, &mut source, Currency::new(String::from("eur")));
    let usd = convert(&my_money, &mut source, Currency::new(String::from("usd")));

    println!("{} PLN = {} EUR", my_money.amount, eur.unwrap());
    println!("{} PLN = {} USD", my_money.amount, usd.unwrap());
}
