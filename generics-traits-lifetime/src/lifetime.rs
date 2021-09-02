
// as we annotated the same lifetime
// the 'a lifetime will be the smallest
// of lifetimes x and y

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This code won't compile since returnable value goes
// out of scope and gets cleaned up at the end of
// the longest function

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//      String::from("string");
// }

// Two lifetimes
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {...}

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
    let s: &'static str = "
        The reference with static can 
        live as long as program goes.
        The text of this string is stored directly in the
        program's binary which is always available
    ";
        3
    }

    // The compiler will give &self and announcement their
    // own lifetimes. Because one of the parameters is 
    // &self, the return type gets the lifetime of &self

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }

}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display 
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

