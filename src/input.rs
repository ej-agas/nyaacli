use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "nyaacli", about = "CLI Bittorrent software for cats (UNOFFICIAL)", author = "ej agas (agas.ej09@gmail.com)")]
pub struct InputBag {
    /// Your query.
    query: String,

    /// Sort by size, date, seeders, leechers, downloads.
    #[structopt(short = "s", long = "sort", default_value = "date")]
    sort: String,

    /// Order by asc (Ascending), desc (Descending).
    #[structopt(short = "o", long = "order", default_value = "desc")]
    order: String
}

impl InputBag {

    pub fn query(&self) -> &String {
        return &self.query;
    }

    pub fn sort_by(&self) -> String {

        let sort = self.sort.as_str();

        return match sort {
            "size" => String::from("size"),
            "date" => String::from("id"),
            "seeders" => String::from("seeders"),
            "leechers" => String::from("leechers"),
            "downloads" => String::from("downloads"),
            _ => panic!("Invalid sort flag {:#?}", sort)
        }
    }

    pub fn order_by(&self) -> String {

        let order = self.order.as_str();

        return match order {
            "desc" => String::from("desc"),
            "asc" => String::from("asc"),
            _ => panic!("Invalid order flag, expecting 'desc' or 'asc', but got {:#?}", order)
        }
    }
}
