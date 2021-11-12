use std::collections::BTreeMap;
use select::document::Document;
use select::predicate::{Attr, Class, Predicate};

#[derive(Debug)]
pub struct Anime {
    pub name: String,
    pub size: String,
    pub seeds: String,
    pub leechers: String,
    pub downloads: String,
    pub date_uploaded: String
}

pub async fn fetch_animes(query: &String, sort: String, order: String) -> BTreeMap<usize, Anime> {
    let client = reqwest::Client::new();
    let response = client.get("https://nyaa.si/?f=0&c=1_2")
        .query(&[("q", query.as_str()), ("s", sort.as_str()), ("o", order.as_str())])
        .send()
        .await
        .unwrap()
        ;

    assert!(response.status().is_success());

    return response_into_hash(&response.text().await.unwrap()).await;
}

async fn response_into_hash(res: &str) -> BTreeMap<usize, Anime> {
    let document = Document::from_read(res.as_bytes()).unwrap();

    let table = document.find(Class("torrent-list")).next().unwrap();

    let mut map = BTreeMap::new();

    for (count, row) in table.find(Class("danger")).enumerate() {
        let name = row.find(Attr("colspan", "2").descendant(Attr("title", ())))
            .last()
            .unwrap()
            .text();

        let size = row.find(Class("text-center"))
            .nth(1)
            .unwrap()
            .text();

        let date = row.find(Class("text-center"))
            .nth(2)
            .unwrap()
            .text();

        let seeds = row.find(Class("text-center"))
            .nth(3)
            .unwrap()
            .text();

        let leechers = row.find(Class("text-center"))
            .nth(4)
            .unwrap()
            .text();

        let downloads = row.find(Class("text-center"))
            .nth(5)
            .unwrap()
            .text();

        let _magnet_link = row.find(Class("text-center"))
            .nth(0)
            .unwrap()
            .children()
            .nth(3)
            .unwrap()
            .attr("href")
            .unwrap();

        map.insert(count, Anime {
            name,
            size,
            seeds,
            leechers,
            downloads,
            date_uploaded: date
        });
    }

    return map;
}
