mod input;
mod nyaa;
mod table;

use crate::input::InputBag;
use std::io::Write;
use structopt::StructOpt;

#[macro_use]
extern crate prettytable;

fn main() {
    let input = InputBag::from_args();

    let animes = &nyaa::fetch_animes(input);

    let table = table::create_table(&animes);
    table.printstd();

    print!("Enter row count to download: ");
    std::io::stdout().flush().unwrap();

    let mut input_ids = String::new();
    std::io::stdin().read_line(&mut input_ids).unwrap();

    let input_ids = input_ids.trim();
    let input_ids = input_ids.split(",").collect::<Vec<_>>();

    println!("{:?}", input_ids);
}
