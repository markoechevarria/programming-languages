mod front_of_house;

/*
mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
*/

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fuit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fuit: String::from("")
            }
        }
    }
}

mod customer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist(); this dont work
        crate::front_of_house::hosting::add_to_waitlist();
        super::front_of_house::hosting::add_to_waitlist();
    }
}

pub use crate::front_of_house::hosting;
use crate::back_of_house::Breakfast;
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn deliver_order() {}
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}
