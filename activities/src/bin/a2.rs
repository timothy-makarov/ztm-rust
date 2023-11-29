// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn print_result(x: i32) {
    println!("{:?}", x);
}

fn main() {
    let x = 1;
    let y = 2;
    let z = add(x, y);
    print_result(z);
}
