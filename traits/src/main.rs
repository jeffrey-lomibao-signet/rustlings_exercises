pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!(
            "{} at {}. (Read more from {}...)",
            self.headline, self.location, self.author
        )
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

    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

fn main() {
    println!("Hello, world!");
    let news_article = NewsArticle {
        headline: "Covid-19".to_string(),
        location: "La Puente".to_string(),
        author: "Jeffrey".to_string(),
        content: "Stay at home!".to_string(),
    };
    println!("{}", news_article.summarize());

    let tweet = Tweet {
        username: "Jeffrey".to_string(),
        content: "Hello!".to_string(),
        reply: true,
        retweet: true,
    };
    println!("{}", tweet.summarize());
}
