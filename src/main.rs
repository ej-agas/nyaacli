mod nyaa;
mod table;
mod input;

use std::io::Write;
use structopt::StructOpt;
use crate::input::InputBag;

#[macro_use] extern crate prettytable;

#[tokio::main]
async fn main() {

    let input = InputBag::from_args();

    println!("{:#?}", input);

    let animes = &nyaa::fetch_animes(
        &input.query(),
        input.sort_by(),
        input.order_by()
    ).await;

    let table = table::create_table(&animes).await;
    table.printstd();

    print!("Enter row count to download: ");
    std::io::stdout().flush().unwrap();

    let mut input_ids = String::new();
    std::io::stdin().read_line(&mut input_ids).unwrap();

    let input_ids = input_ids.trim();
    let input_ids = input_ids.split(",").collect::<Vec<_>>();

    println!("{:?}", input_ids);
}

