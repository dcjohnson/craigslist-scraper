use std::vec::Vec;

use hyper::Client;
use hyper::header::Connection;

use std::io::Read;
use std::default::Default;

use tendril::StrTendril;

use html5ever::tokenizer::{Tokenizer, TokenizerOpts};

fn get_html(url: &str) -> StrTendril {
    let mut res = Client::new().get(url)
      .header(Connection::close()).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let input = StrTendril::try_from_byte_slice(body.as_bytes()).unwrap();
    input.try_reinterpret().unwrap()
}

pub fn get_rows(url: &str) -> Vec<String> {
    let mut tok = Tokenizer::new(collector::RowCollector::new(), TokenizerOpts {
        .. Default::default()
    });
    tok.feed(get_html(url));
    tok.end();
    tok.unwrap().rows
}

mod collector {
    use std::vec::Vec;
    use std::str::FromStr;

    use html5ever::tokenizer::{TokenSink, Token, Tag};
    use html5ever::tokenizer::{TagToken, StartTag, EndTag};

    pub struct RowCollector {
        pub rows: Vec<String>
    }

    impl RowCollector {
        pub fn new() -> Self {
            RowCollector {
                rows: Vec::new(),
            }
        }

        fn get_url(&self, tag: Tag) -> Option<String> {
            if match tag.kind {
                StartTag => true,
                EndTag => false
            } && tag.name.as_ref() == "p" {
                for attr in tag.attrs.iter() {
                    if attr.name.local.as_ref() == "data-pid" {
                        return Some(String::from_str(attr.value.as_ref()).unwrap())
                    }
                }
            }

            return None
        }
    }

    impl TokenSink for RowCollector {
        fn process_token(&mut self, token: Token) {
            match token {
                TagToken(tag) => {
                    let url = self.get_url(tag);
                    if url.is_some() {
                        self.rows.push(url.unwrap());
                    }
                }
                _ => { }
            }
        }
    }
}
