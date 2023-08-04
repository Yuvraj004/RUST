#![ allow(unused)]
mod front_of_house { //declaring a module syntax
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod hosting_public {
        pub fn add_to_waitlist_public() {}
    }
    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}
pub fn eat_at_res(){
    //abs path
    // crate::front_of_house :: hosting::add_to_waitlist();
    //cannot be accessed becoz private
    //relative path
    front_of_house :: hosting_public::add_to_waitlist_public();

}
fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();//TO INCLUDE A FUNCTION OUTSIDE MODULE
    }
    fn cook_order() {}
    //privacy rules for modules
    pub struct Breakfast {
        toast: String,
        seasonal_fruit: String,
        pub toasty : String,
    }
    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast { 
                toast: String:: from (toast),
                seasonal_fruit: String::from("peaches"),
                toasty: String::from("tasty"),
            }
        }
    }
    //1. berakfast struct & summer function private by default
    //2. toast is private even though breakfast is public
    //3. cannot create struct directly in main fn becoz seasonalfruit field is private
}
pub fn eat_at_res2(){
    let mut meal = back_of_house::Breakfast::summer("Pre");
    //lets try reassigning meal
    //error becos private variable
    //meal.toast =String::from("wheat");
    meal.toasty =String::from("wheat");

}
//
//
//making enums public will make its all elements public unlike struct
mod back_of_house_enum {
    pub enum Appetizer {
        Soup,
        Salad,
        }
}
pub fn eat_at_restaurant_enum() {
    let order1: back_of_house_enum::Appetizer = back_of_house_enum::Appetizer::Soup;

    let order2: back_of_house_enum::Appetizer = back_of_house_enum::Appetizer::Salad;
    //no need to make Soup/Salad public separately 
}
//
//
//use keyword to access functions in modules
//USE is used to bring module/external dependencies into our working scope
use rand::{Rng,CryptoRng,ErrorKind::Transient};
use std::io::{self,Write};
use std::io::*;//to import all modules
//
//
//USING MODULES FROM dIFFERENT FILES WITH SAME NAME
mod front_of_house_use;
pub use self:: front_of_house_use::hosting;// pub used so that external modules can also access above module
//we do not add functions in the use keyword becoz many modules contain same
//name of certain functions 
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
