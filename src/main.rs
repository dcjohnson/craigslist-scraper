extern crate lib;
extern crate getopts;

use std::env;
use std::thread;
use std::time::Duration;

use lib::row_collector;
use lib::url::Url;

use getopts::Options;
use getopts::Matches;

fn main() {
    let matches = get_options(env::args().collect());
    let dur = if matches.opt_present("d") {
        matches.opt_default("d", "5").unwrap().parse::<u64>().unwrap()
    } else {
        5
    } * 60;
    let loc = matches.opt_str("l").unwrap();
    let topic = matches.opt_str("t").unwrap();
    let query = matches.opt_strs("q").join("&");
    let url = Url::new(loc, topic, query);
    let mut ids: Vec<String> = Vec::new();

    loop {
        let new_ids = row_collector::get_rows(url.get_search_url());
        for id in new_ids.iter() {
            if ids.len() == 0 || id.as_str() == ids.first().unwrap().as_str() {
                break
            } else {
                println!("{}", url.make_post_url(id));
            }
        }
        ids = new_ids;
        thread::sleep(Duration::new(dur, 0));
    }
}

fn get_options(args: Vec<String>) -> Matches {
    let mut opts = Options::new();
    opts.reqopt("l", "location", "<location>.craigslist.org", "provo");
    opts.reqopt("t", "topic", "provo.craigslist.org/search/<topic>", "roo");
    opts.optmulti("q", "query", "provo.craigslist.org/search/roo?<query>", "sort=priceasc");
    opts.optopt("d", "duration", "interval that the scrapes are made at", "");
    opts.parse(&args[1..]).unwrap()
}
