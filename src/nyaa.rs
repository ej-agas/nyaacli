use crate::input::InputBag;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Anime {
    pub name: String,
    pub size: String,
    pub seeds: String,
    pub leechers: String,
    pub downloads: String,
    pub date_uploaded: String,
    pub magnet_link: String,
}

pub fn fetch_animes(input: InputBag) -> BTreeMap<usize, Anime> {
    let response = ureq::get("https://nyaa.si")
        .query("f", input.filter().as_str())
        .query("c", input.category().as_str())
        .query("q", input.query().as_str())
        .query("s", input.sort_by().as_str())
        .query("o", input.order_by().as_str())
        .call()
        .expect("Failed to connect to nyaa, are you connected to the internet?")
        .into_string()
        .unwrap();

    return response_into_hash(&response);
}

fn response_into_hash(res: &str) -> BTreeMap<usize, Anime> {
    let document = Document::from_read(res.as_bytes()).unwrap();

    let table = document
        .find(Class("torrent-list"))
        .next()
        .expect("Nyaa returned 0 results.");

    let table = table.find(Name("tbody")).next().unwrap();

    let mut map = BTreeMap::new();

    for (count, row) in table.find(Name("tr")).enumerate() {
        let name = row
            .find(Attr("colspan", "2").descendant(Attr("title", ())))
            .last()
            .unwrap()
            .text();

        let size = row.find(Class("text-center")).nth(1).unwrap().text();

        let date = row.find(Class("text-center")).nth(2).unwrap().text();

        let seeds = row.find(Class("text-center")).nth(3).unwrap().text();

        let leechers = row.find(Class("text-center")).nth(4).unwrap().text();

        let downloads = row.find(Class("text-center")).nth(5).unwrap().text();

        let magnet_link = String::from(
            row.find(Class("text-center"))
                .next()
                .unwrap()
                .children()
                .nth(3)
                .unwrap()
                .attr("href")
                .unwrap(),
        );

        map.insert(
            count,
            Anime {
                name,
                size,
                seeds,
                leechers,
                downloads,
                date_uploaded: date,
                magnet_link,
            },
        );
    }

    return map;
}
