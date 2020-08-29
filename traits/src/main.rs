mod lib;
use lib::Summary;

fn main() {
    let tweet = lib::Tweet {
        username: String::from("andrew"),
        content: String::from("content"),
        reply: true,
        retweet: true,
    };
    println!("{:#?}", tweet);
    println!("{}", tweet.summarize());
}
