pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Trait bound syntax, here are all parameters forced
// to the same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     ...
// }

// if we need to implement multiple traits:
// pub fn notify(item: &(impl Summary + Display)) {...}
// the same can be done with trait bounds

// Where clauses
// instead of:
// fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// we can do
// fn some_func<T, U>(t: &T, u: &U) -> i32
//    where T: Display + Clone,
//          U: Clone + Debug
// {...}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    //fn summarize(&self) -> String {
        //format!("{}, by {} ({})", self.headline, self.author, self.location)
    //}

    // this is implementation of the method from trait
    // however if it will be commented then default 
    // implementation will be applied
    
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub replay: bool,
    pub retweet: bool,
}

// This function returns some type 
// that implements Summary trait
//fn return_summarazible() -> impl Summary {    
   // Tweet {
     //   username: String::from("user"),
       // content: String::from("Hello, World!"),
        //replay: false,
       // retweet: false,
    //};
//}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    //fn summarize(&self) -> String {
       // format!("{}: {}", self.username, self.content)
    //}
}
