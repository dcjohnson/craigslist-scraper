extern crate lib;

use std::env;

use lib::row_collector;
use lib::flag_parser;

fn main() {
    let fp = flag_parser::FlagParser::new(env::args().collect());
    for row in row_collector::get_rows("https://provo.craigslist.org/search/roo?sort=date").iter() {
        println!("{} {}", row, fp.make_post_url(row));
    }
    println!("{}", fp.search_url);
}
