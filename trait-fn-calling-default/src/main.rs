mod tweets;
use crate::tweets::Summary;

fn main() {
    let tweet1 = tweets::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let tweet2 = tweets::Tweet {
        username: String::from("cat_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new notification: {}", tweet1.summarize());
    println!("1 new notification: {}", tweet2.summarize());
}