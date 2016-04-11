extern crate lib;
extern crate getopts;
extern crate rustc_serialize;

use std::env;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::fs::File;

use rustc_serialize::json;

use lib::row_collector;
use lib::url::Url;
use lib::slack::SlackClient;

use getopts::Options;
use getopts::Matches;

fn main() {
    let matches = get_options(env::args().collect());
    let dur = if matches.opt_present("d") {
        matches.opt_default("d", "5").unwrap().parse::<u64>().unwrap()
    } else {
        5
    } * 60;
    let url = Url::new(matches.opt_str("l").unwrap(), matches.opt_str("t").unwrap(), matches.opt_strs("q").join("&"));
    let slack_client = get_slack_client(matches.opt_str("c").unwrap());
    let mut ids: Vec<String> = Vec::new();
    slack_client.send_post("test".to_string());
    println!("msg sent");
    loop {
        let new_ids = row_collector::get_rows(url.get_search_url());
        let mut post_vector: Vec<String> = Vec::new();
        for id in new_ids.iter() {
            if ids.len() == 0 || id.as_str() == ids.first().unwrap().as_str() {
                break
            } else {
                post_vector.push(url.make_post_url(id));
            }
        }
        ids = new_ids;
        if post_vector.len() > 0 {
            slack_client.send_post(post_vector.join("\n"));
        }
        thread::sleep(Duration::new(dur, 0));
    }
}

fn get_slack_client(path: String) -> SlackClient {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    json::decode(&buf).unwrap()
}

fn get_options(args: Vec<String>) -> Matches {
    let mut opts = Options::new();
    opts.reqopt("l", "location", "<location>.craigslist.org", "provo");
    opts.reqopt("t", "topic", "provo.craigslist.org/search/<topic>", "roo");
    opts.reqopt("c", "config", "slack configuration file", "/path/to/file");
    opts.optmulti("q", "query", "provo.craigslist.org/search/roo?<query>", "sort=priceasc");
    opts.optopt("d", "duration", "interval that the scrapes are made at", "");
    opts.parse(&args[1..]).unwrap()
}
