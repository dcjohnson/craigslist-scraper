extern crate lib;

use lib::row_collector;

fn main() {
    for row in row_collector::get_rows("https://provo.craigslist.org/search/roo?sort=date").iter() {
        println!("{}", row);
    }
}
