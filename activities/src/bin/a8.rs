// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Drink {
    Vanilla,
    Chocolate,
    Mango,
}

struct Drinks {
    flavor: Drink,
    amount: i32
}

fn print_drink(drink: Drinks){
    match drink.flavor {
        Drink::Vanilla => println!("{}ml Vanilla drink", drink.amount),
        Drink::Chocolate => println!("{}ml Chocolate drink", drink.amount),
        Drink::Mango => println!("{}ml Mango drink", drink.amount),
    }
}

fn main() {
    let my_drink = Drinks {
        flavor: Drink::Chocolate,
        amount: 200
    };

    print_drink(my_drink);
}
