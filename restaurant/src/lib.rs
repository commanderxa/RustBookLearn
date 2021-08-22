mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_food: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_food: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soap,
        Salad,
    }
}

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {

    // crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();


    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::soap;
    let order2 = back_of_house::Appetizer::salad;
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}

fn function2() -> IoResult<()> {}
