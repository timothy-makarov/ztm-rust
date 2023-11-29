// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItemInfo {
    id: i32,
    quatity: i32
}

fn print_gi_id(item: &GroceryItemInfo) {
    println!("ID is {:?}", item.id);
}

fn print_gi_quatity(item: &GroceryItemInfo) {
    println!("Quatity is {:?}", item.quatity);
}

fn main() {
    let item = GroceryItemInfo {
        id: 1,
        quatity: 5
    };

    print_gi_id(&item);
    print_gi_quatity(&item);
}
