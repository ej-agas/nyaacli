use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nyaacli",
    about = "CLI bittorrent software for nerd cats.",
    author = "ej-agas (agas.ej09@gmail.com)"
)]
pub struct InputBag {
    /// Your query.
    query: String,

    /// Sort by size, date, seeders, leechers, downloads.
    #[structopt(short = "s", long = "sort", default_value = "date")]
    sort: String,

    /// Order by asc (Ascending), desc (Descending).
    #[structopt(short = "o", long = "order", default_value = "desc")]
    order: String,

    /**
       0 No Filter

       1 No Remakes

       2 Trusted Only
    */
    #[structopt(short = "f", long = "filter", default_value = "0")]
    filter: String,

    /**
       0 All Categories

       1 Anime

       2 Anime Music Video

       3 Anime English-Translated

       4 Anime Non-English-Translated

       5 Anime Raw

       6 Audio

       7 Audio Lossless

       8 Audio Lossy

       9 Literature

       10 Literature English-Translated

       11 Literature Non-English-Translated

       12 Literature Raw

       13 Live Action

       14 Live Action English-Translated

       15 Live Action Idol/Promotional Video

       16 Live Action Non-English-Translated

       17 Live Action Raw

       18 Pictures

       19 Pictures Graphics

       20 Pictures Photos

       21 Software

       22 Software Applications

       23 Software Games

    */
    #[structopt(short = "c", long = "category", default_value = "0")]
    category: String,
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
            _ => panic!("Invalid sort flag {:#?}", sort),
        };
    }

    pub fn order_by(&self) -> String {
        let order = self.order.as_str();

        return match order {
            "desc" => String::from("desc"),
            "asc" => String::from("asc"),
            _ => panic!(
                "Invalid order flag, expecting 'desc' or 'asc', but got {:#?}",
                order
            ),
        };
    }

    pub fn filter(&self) -> String {
        let filter = self.filter.as_str();

        return match filter {
            "0" => String::from("0"),
            "1" => String::from("1"),
            "2" => String::from("2"),
            _ => String::from("0"),
        };
    }

    pub fn category(&self) -> String {
        let category = self.category.as_str();

        return match category {
            "0" => String::from("0_0"),
            // Anime
            "1" => String::from("1_0"),
            "2" => String::from("1_1"),
            "3" => String::from("1_2"),
            "4" => String::from("1_3"),
            "5" => String::from("1_4"),
            // Audio
            "6" => String::from("2_0"),
            "7" => String::from("2_1"),
            "8" => String::from("2_2"),

            // Literature
            "9" => String::from("3_0"),
            "10" => String::from("3_1"),
            "11" => String::from("3_2"),
            "12" => String::from("3_3"),

            // Live Action
            "13" => String::from("4_0"),
            "14" => String::from("4_1"),
            "15" => String::from("4_2"),
            "16" => String::from("4_3"),
            "17" => String::from("4_4"),

            // Pictures
            "18" => String::from("5_0"),
            "19" => String::from("5_1"),
            "20" => String::from("5_2"),

            // Software
            "21" => String::from("6_0"),
            "22" => String::from("6_1"),
            "23" => String::from("6_2"),

            _ => String::from("0_0"),
        };
    }
}
