extern crate lib;
extern crate getopts;
extern crate slack;

use std::env;
use std::thread;
use std::time::Duration;
use lib::row_collector;
use lib::url::Url;
use getopts::Options;
use slack::RtmClient;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = get_options();
    if args.len() == 1 {
        println!("{}", options.usage("Scraper usage"));
        std::process::exit(0);
    }
    let matches = options.parse(&args[1..]).unwrap();
    let dur = if matches.opt_present("d") {
        matches.opt_default("d", "5").unwrap().parse::<u64>().unwrap()
    } else {
        5
    } * 60;
    let url = Url::new(matches.opt_str("l").unwrap(), matches.opt_str("t").unwrap(), matches.opt_strs("q").join("&"));
    let channel = format!("#{}", matches.opt_str("c").unwrap().as_str());
    let mut slack_client = RtmClient::new(matches.opt_str("b").unwrap().as_str());
    let login_result = slack_client.login();
    if login_result.is_err() {
        panic!("login failed");
    }
    let mut ids: Vec<String> = Vec::new();
    loop {
        let new_ids = row_collector::get_rows(url.get_search_url());
        let mut post_vector: Vec<String> = Vec::new();
        for id in new_ids.iter() {
            if ids.len() == 0 || id.as_str() == ids.first().unwrap().as_str() {
                break;
            } else {
                post_vector.push(url.make_post_url(id));
            }
        }
        ids = new_ids;
        if post_vector.len() > 0 {
            let post_result =
                slack_client.post_message(channel.as_str(), post_vector.join("\n").as_str(), None);
            if post_result.is_err() {
                println!("failed to post to slack")
            }
        }
        thread::sleep(Duration::new(dur, 0));
    }
}

fn get_options() -> Options {
    let mut opts = Options::new();
    opts.reqopt("l", "location", "<location>.craigslist.org", "provo");
    opts.reqopt("t", "topic", "provo.craigslist.org/search/<topic>", "roo");
    opts.reqopt("b", "bot-token", "slack bot token", "");
    opts.reqopt("c", "post-channel", "slack channel to post to", "#<channel>");
    opts.optmulti("q", "query", "provo.craigslist.org/search/roo?<query>", "sort=priceasc");
    opts.optopt("d", "duration", "interval that the scrapes are made at", "1min mininum");
    opts
}
