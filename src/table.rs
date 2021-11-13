use crate::nyaa::Anime;
use prettytable::format::consts::FORMAT_BOX_CHARS;
use prettytable::Table;
use std::collections::BTreeMap;

pub fn create_table(animes: &BTreeMap<usize, Anime>) -> Table {
    let mut table = Table::new();
    table.set_format(*FORMAT_BOX_CHARS);
    table.set_titles(row![
        "",
        "Name",
        "Size",
        "Seeds",
        "Leechers",
        "Downloads",
        "Date Uploaded"
    ]);

    for (key, val) in animes {
        table.add_row(row![
            key,
            truncate(&val.name, 100),
            val.size,
            Fg->val.seeds,
            Fr->val.leechers,
            val.downloads,
            val.date_uploaded
        ]);
    }

    return table;
}

fn truncate(anime_name: &str, max_chars: usize) -> String {
    let mut anime_name = String::from(anime_name);
    if anime_name.char_indices().count() > max_chars {
        let length = truncate_by_index(&anime_name, max_chars);
        anime_name.truncate(length);

        return format!("{}...", anime_name);
    }

    return anime_name;
}

fn truncate_by_index(anime_name: &str, max_chars: usize) -> usize {
    match anime_name.char_indices().nth(max_chars) {
        None => anime_name.len(),
        Some((idx, _)) => anime_name[..idx].len(),
    }
}
