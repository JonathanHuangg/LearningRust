// traits group method signatures together to define behavior
// can also be used as paremeters (obviously)
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// can have no concrete implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        formate!("{}: {}", self.username, self.content)
    }
}


fn main() {
    let tweet = Tweet{
        username: String::from("ItsXNow"),
        content: String::from(
            "Something something something"
        ),
        reply: false,
        retweet: false,
    };

    pritnln!("1 new tweet: {}", tweet.summarize());
}