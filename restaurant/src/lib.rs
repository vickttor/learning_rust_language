#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("Hello World");
        }
        pub fn seat_at_table(){
            println!("What do you wanna Sr.?");
        }
    }
   pub mod serving{
       pub fn take_order(){
         crate::front_of_house::hosting::seat_at_table();
         //We can use super:: syntax to specify a parent path:
         super::hosting::add_to_waitlist();
       }
        fn serve_order(){}
        fn take_payments(){}
    }
}

fn take_function_of_module(){
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::serving::take_order();
}

//We can also use pub to designate structs and enums as public, but there are a
//few extra details. If we use pub before a struct definition, we make the struct 
//public, but the struct’s fields will still be private. We can make each field
//public or not on a case-by-case basis. In Listing 7-9, we’ve defined a public
//back_of_house::Breakfast struct with a public toast field but a private 
//seasonal_fruit field. This models the case in a restaurant where the customer 
//can pick the type of bread that comes with a meal, but the chef decides which 
//fruit accompanies the meal based on what’s in season and in stock. The available 
//fruit changes quickly, so customers can’t choose the fruit or even see which 
//fruit they’ll get.






mod back_of_house {
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

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}


//Because the toast field in the back_of_house::Breakfast struct is public,
//in eat_at_restaurant we can write and read to the toast field using dot notation.
//Notice that we can’t use the seasonal_fruit field in eat_at_restaurant because 
//seasonal_fruit is private. Try uncommenting the line modifying the seasonal_fruit 
//field value to see what error you get!
// 
//Also, note that because back_of_house::Breakfast has a private field, the struct
//needs to provide a public associated function that constructs an instance of 
//Breakfast (we’ve named it summer here). If Breakfast didn’t have such a
//function, we couldn’t create an instance of Breakfast in eat_at_restaurant
//because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.

mod back_of_house_2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_2(){
    let order1 = back_of_house_2::Appetizer::Soup;
    let order2 = back_of_house_2::Appetizer::Salad;
}

//In contrast, if we make an enum public, all of its variants are then public.
//We only need the pub before the enum keyword.
//
//Because we made the Appetizer enum public, we can use the Soup and Salad variants 
//in eat_at_restaurant. Enums aren’t very useful unless their variants are public; 
//it would be annoying to have to annotate all enum variants with pub in every case, 
//so the default for enum variants is to be public. Structs are often useful without 
//their fields being public, so struct fields follow the general rule of everything
//being private by default unless annotated with pub.
//
//There’s one more situation involving pub that we haven’t covered, and that is our
//last module system feature: the use keyword. We’ll cover use by itself first, and 
//then we’ll show how to combine pub and use.



