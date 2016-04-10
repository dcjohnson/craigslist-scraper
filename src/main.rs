extern crate lib;

use std::env;

use lib::row_collector;
use lib::flag_parser;

fn main() {
    let fp = flag_parser::FlagParser::new(env::args().collect());
    for row in row_collector::get_rows(fp.get_search_url()).iter() {
        println!("{} {}", row, fp.make_post_url(row));
    }
    println!("{}", fp.get_search_url());
}
