use std::collections::BTreeMap;
use prettytable::Table;
use prettytable::format::consts::FORMAT_BOX_CHARS;
use crate::nyaa::Anime;


pub async fn create_table(animes: &BTreeMap<usize, Anime>) -> Table {
    let mut table = Table::new();
    table.set_format(*FORMAT_BOX_CHARS);
    table.set_titles(row!["", "Name", "Size", "Seeds", "Leechers", "Downloads", "Date Uploaded"]);

    for (key, val) in animes {
        table.add_row(row![
            key,
            val.name,
            val.size,
            Fg->val.seeds,
            Fr->val.leechers,
            val.downloads,
            val.date_uploaded
        ]);
    }

    return table
}

fn limit_length(name: &str, numchars: usize) -> String {
    let mut anime_name = String::from(name);

    if anime_name.chars().count() > numchars {
        return String::from(name);
    }

    return anime_name;
}
