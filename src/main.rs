extern crate lib;
extern crate getopts;

use std::env;

use lib::row_collector;
use lib::url;

use getopts::Options;
use getopts::Matches;

fn main() {
    let matches = get_options(env::args().collect());
    let loc = matches.opt_str("l").unwrap();
    let topic = matches.opt_str("t").unwrap();
    let query = matches.opt_strs("q").join("&");

    let fp = url::Url::new(loc, topic, query);
    for row in row_collector::get_rows(fp.get_search_url()).iter() {
        println!("{} {}", row, fp.make_post_url(row));
    }
    println!("{}", fp.get_search_url());
}

fn get_options(args: Vec<String>) -> Matches {
    let mut opts = Options::new();
    opts.reqopt("l", "location", "<location>.craigslist.org", "provo");
    opts.reqopt("t", "topic", "provo.craigslist.org/search/<topic>", "roo");
    opts.optmulti("q", "query", "provo.craigslist.org/search/roo?<query>", "sort=priceasc");
    opts.optopt("d", "duration", "interval that the scrapes are made at", "");
    opts.parse(&args[1..]).unwrap()
}
