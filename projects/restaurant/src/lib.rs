mod front_of_house {

    #[allow(dead_code)]
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    #[allow(dead_code)]
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_the_house {

    pub enum Appetizer {
        Soup,
        Salad,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use crate::front_of_house::hosting;

#[allow(unused_variables)]
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = back_of_the_house::Breakfast::summer("French");
    meal.toast = String::from("Wheat");
    println!("{meal:?}");

    let order1 = back_of_the_house::Appetizer::Soup;
    let order1 = back_of_the_house::Appetizer::Salad;
}
