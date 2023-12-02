// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    color: String
}

fn print_string(value: &str) {
    println!("{:?}", value);
}

fn main() {
    let persons = vec![
        Person {
            name: "John".to_owned(),
            age: 33,
            color: "Red".to_owned()
        },
        Person {
            name: "Mary".to_owned(),
            age: 8,
            color: "Green".to_owned()
        },
        Person {
            name: "Xi".to_owned(),
            age: 70,
            color: "Blue".to_owned()
        }
    ];

    for p in persons {
        if p.age <= 10 {
            print_string(&p.name);
            print_string(&p.color);
        }
    }
}
