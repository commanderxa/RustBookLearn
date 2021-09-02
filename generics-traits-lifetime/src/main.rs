mod generics;
mod traits;
mod lifetime;

use traits::Summary;

fn main() {
    println!("Hello, world!");

    let num_list = [10, 400, 20, 32, -10, 77];
    let result = generics::largest(&num_list);
    println!("The largest num is: {}", result);

    let result = generics::largest_v2(&num_list);
    println!("The largest num from second func is: {}", result);

    let integer = generics::PointOld { x: 5, y: 10, };
    let float = generics::PointOld { x: 1.0, y: 4.0, };
    let mixed = generics::Point { x: 5, y: 4.0, };

    let p = generics::PointOld { x: 5, y: 10, };
    println!("p.x = {}", p.x());

    let p1 = generics::Point { x: 5, y: 10.4, };
    let p2 = generics::Point { x: "Hello", y: 'c', };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = traits::Tweet {
        username: String::from("user"),
        content: String::from("Hello, World!"),
        replay: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    let article = traits::NewsArticle {
        headline: String::from("Debian 11 bullseye released"),
        location: String::from("San-Francisco, CA, USA"),
        author: String::from("The Debian Project"),
        content: String::from(
            "After a lot of time of testing the new 
            version of linux distribution Debian, the team
            officially release the 11th version of Debian"       ),
    };
    
    
    println!("New article available! {}", article.summarize());

    let novel = String::from("Something here. And here.");
    let first_sentence = novel.split('.').next().expect("Couldn't find '.'");
    let i = lifetime::ImportantExcerpt {
        part: first_sentence
    };
}
