pub struct Url {
    location: String,
    topic: String,
    search_url: String
}

impl Url {
    pub fn new(loc: String, topic: String, query: String) -> Self {
        let search_url = format!("https://{location}.craigslist.org/search/{topic}?{query}", location=loc, topic=topic, query=query);
        Url {
            location: loc,
            topic: topic,
            search_url: search_url
        }
    }

    pub fn make_post_url(&self, id: &String) -> String {
        // the angle brackets are so that it shows up in slack as a link
        format!("<https://{location}.craigslist.org/{topic}/{id}.html>", location=self.location, topic=self.topic, id=id)
    }

    pub fn get_search_url(&self) -> &String {
        &self.search_url
    }
}
