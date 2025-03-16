/* the Rust [module] system
 *  package (a [Cargo] feature for building/testing/sharing [crates])
 *  crate   (a [tree] of [modules] which produces a [library/executable] (e.g. lib.rs main.rs))
 *  module  (a way of controlling the [organization/scope/privacy] of paths)
 *  path    (a way of naming [structs/funtions/modules] etc.)
*/
mod back_end {
    // The default for [enum] variants is to be public.
    pub enum Appetizer {
        Soup(String),
        Salad(String),
    }

    // The default for [struct] fields is to be private,
    // unless annotated with [pub].
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Since the [struct] has a private field, it has to provide
    // a public association function to construct instances.
    impl Breakfast {
        pub fn autumn(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("kiwi"),
            }
        }
    }
}

mod front_end {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("\nadded to waitlist");
        }
        pub fn get_feedback() {
            println!("got feedbacks for hosting")
        }
    }
    pub mod serving {
        pub fn take_order() {
            println!("orders taken");
        }
        pub fn get_feedback() {
            println!("got feedbacks for serving")
        }
    }
}

mod customer {
    pub mod reservation {
        pub fn take_out() {
            println!("packed up to go");
        }
        pub fn dine_in() {
            // Since [use] only creates the shortcut for the exact scope in which
            // it occurs, here within the [child] modules, we reference the shortcut
            // in the [parent] module with [super]
            super::super::hosting::add_to_waitlist();
            super::super::serving::take_order();
        }
    }
}

// (1) Bring paths into scope with the [use] keyword;
use crate::front_end::hosting;
use front_end::serving; // relative path; [self] omitted // absolute path;

// (2) Provide aliases using the [as] keyword to avoid the same-name conflict
use front_end::hosting::get_feedback as get_hosting_feedback;
use front_end::serving::get_feedback as get_serving_feedback;

// (3) Use [nested paths] to clean up large use lists
use customer::reservation::{self, dine_in};

fn main() {
    let mut meal = back_end::Breakfast::autumn("Rye");
    meal.toast = String::from("Wheat");
    println!("\nI'd like {:#?} - {} as toast - please", meal, meal.toast);

    // (1)
    hosting::add_to_waitlist();
    serving::take_order();

    // (2)
    get_serving_feedback();
    get_hosting_feedback();

    // (3)
    dine_in();
    reservation::take_out();
}
