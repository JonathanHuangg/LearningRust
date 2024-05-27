pub fn add(left: usize, right: usize) -> usize {
    left + right
}



mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn front_function() {
    hosting::add_to_waitlist();
}


mod customer {
    pub fn eat_at_restaurant() {

        // absolute path
        crate::front_of_house::hosting::add_to_waitlist();
    
        // relative path
        front_of_house::hosting::add_to_waitlist();

        // using use shortcut, use 'pub use' to allow other modules to bring it into scope
        use crate::front_of_house::hosting;

        hosting::add_to_waitlist();
    
        let mut meal = back_of_house::Breakfast::summer("Rye");
    
        meal.toast = String::from("Wheat");
    
        println!("The meal is {}", meal.toast);
    
        let order1 = back_of_house::Appetizer::Soup;
    }
}


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super keyword uses path from parent module
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }

    }

    pub enum Appetizer {
        Soup, 
        Salad,
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
