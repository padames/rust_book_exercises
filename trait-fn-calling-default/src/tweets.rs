pub trait Summary {
    // must be defined by implementors to use `summarize`
    fn summarize_author(&self) -> String;

    // default definition relies on `summarize_author`
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        if self.username == "horse_ebooks" {
            format!("Read more from ")
        } else {
            // Summary::summarize()
            format!("no one")
        }
    }
}
