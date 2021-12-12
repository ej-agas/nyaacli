mod input;
mod nyaa;
mod table;
mod torrent;

use crate::input::InputBag;
use std::io::Write;
use structopt::StructOpt;

#[macro_use]
extern crate prettytable;

fn main() {
    let input = InputBag::from_args();

    let animes = &nyaa::fetch_animes(input);

    let table = table::create_table(animes);
    table.printstd();

    println!("Ex. 0,1,2");
    print!("Enter row count(s) to download: ");
    std::io::stdout().flush().unwrap();

    let mut rows_as_str = String::new();
    std::io::stdin().read_line(&mut rows_as_str).unwrap();

    torrent::download(rows_as_str, animes)
}
