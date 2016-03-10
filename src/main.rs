extern crate html5ever;
extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;

fn main() {
  let mut res = Client::new().get("https://provo.craigslist.org/search/roo?sort=date")
    .header(Connection::close()).send().unwrap();
  let mut body = String::new();
  res.read_to_string(&mut body).unwrap();
  println!("{}", body);
}
