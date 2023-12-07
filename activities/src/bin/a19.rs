// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    store.insert("Chair", 5);
    store.insert("Bed", 3);
    store.insert("Table", 2);
    store.insert("Couch", 0);

    let mut sum = 0;
    for (k,v) in store.iter() {
        if *v == 0 {
            println!("{:?}: out of stock", k);
        } else {
            println!("{:?} {:?}", v, k);
        }
        sum += v;
    }

    println!("Total number: {:?}", sum);
}
