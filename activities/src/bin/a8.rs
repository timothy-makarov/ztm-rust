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

enum DrinkFlavor {
    Orange,
    Apple,
    Chocolate,
    Cola
}

struct DrinkInfo {
    flavor: DrinkFlavor,
    volume: f32
}

fn print_drink_info(drink: DrinkInfo) {
    match drink.flavor {
        DrinkFlavor::Orange     => println!("{} ounces of Orange", drink.volume),
        DrinkFlavor::Apple      => println!("{} ounces of Apple", drink.volume),
        DrinkFlavor::Chocolate  => println!("{} ounces of Chocolate", drink.volume),
        DrinkFlavor::Cola       => println!("{} ounces of Cola", drink.volume)
    }
}

fn main() {
    let drink_1 = DrinkInfo {
        flavor: DrinkFlavor::Cola,
        volume: 10.0
    };
    print_drink_info(drink_1);

    let drink_2 = DrinkInfo {
        flavor: DrinkFlavor::Orange,
        volume: 20.0
    };
    print_drink_info(drink_2);
}
