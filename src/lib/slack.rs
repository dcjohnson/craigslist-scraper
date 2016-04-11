use rustc_serialize::json;

use hyper::Client;
use hyper::header::ContentType;

use std::io::Read;

static SLACK_URL: &'static str = "https://slack.com/api/chat.postMessage";

#[derive(RustcDecodable)]
pub struct SlackClient {
    token: String,
    channel: String,
}

impl SlackClient {
    pub fn send_post(&self, urls: String) {
        println!("post");
        let j = json::encode(&SlackPost {
            token: self.token.clone(),
            channel: self.channel.clone(),
            text: urls,
            as_user: true
        }).unwrap();
        println!("{}", j);
        let mut res = Client::new().post(SLACK_URL).header(ContentType::json()).body(j.as_str()).send().unwrap();
        let mut buf = String::new();
        res.read_to_string(&mut buf).unwrap();
        println!("{}", buf);
        // Client::new().post(SLACK_URL).header(ContentType::json()).body(json::encode(&SlackPost {
        //     token: self.token.clone(),
        //     channel: self.channel.clone(),
        //     text: urls
        // }).unwrap().as_str()).send().unwrap();
    }
}

#[derive(RustcEncodable)]
struct SlackPost {
    token: String,
    channel: String,
    text: String,
    as_user: bool
}
