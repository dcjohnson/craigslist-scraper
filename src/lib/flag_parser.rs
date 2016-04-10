use getopts::Options;

pub struct FlagParser {
    location: String,
    topic: String,
    search_url: String
}

impl FlagParser {
    pub fn new(args: Vec<String>) -> Self {
        let opts = get_options();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(err) => panic!(err.to_string())
        };
        let loc = matches.opt_str("l").unwrap();
        let topic = matches.opt_str("t").unwrap();
        let query = matches.opt_strs("q").join("&");
        let search_url = format!("https://{location}.craigslist.org/search/{topic}?{query}", location=loc, topic=topic, query=query);
        FlagParser {
            location: loc,
            topic: topic,
            search_url: search_url
        }
    }

    pub fn make_post_url(&self, id: &String) -> String {
        format!("https://{location}.craigslist.org/{topic}/{id}.html", location=self.location, topic=self.topic, id=id)
    }

    pub fn get_search_url(&self) -> &String {
        &self.search_url
    }
}

fn get_options() -> Options {
    let mut opts = Options::new();
    opts.reqopt("l", "location", "<location>.craigslist.org", "provo");
    opts.reqopt("t", "topic", "provo.craigslist.org/search/<topic>", "roo");
    opts.optmulti("q", "query", "provo.craigslist.org/search/roo?<query>", "sort=priceasc");
    opts
}
