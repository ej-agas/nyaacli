use std::{collections::BTreeMap, process::Command};

use crate::nyaa::Anime;

pub fn download(rows_as_str: String, animes: &BTreeMap<usize, Anime>) {
    let delimiter = ',';
    let rows_as_str = rows_as_str.trim().trim_end_matches(delimiter);
    let rows = rows_as_str.split(delimiter).collect::<Vec<_>>();

    for index in rows {
        let index = index.parse::<usize>().expect("Enter a valid number.");
        let magnet_link = &animes.get(&index).expect("Row not found.").magnet_link;

        Command::new("xdg-open")
            .arg(magnet_link)
            .spawn()
            .expect("Command failed. Do you have xdg-utils installed?");
    }
}
