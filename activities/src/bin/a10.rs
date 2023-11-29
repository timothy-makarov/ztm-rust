// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_result(res: bool) {
    let msg = match res {
        true => "its big",
        false => "its small"
    };
    println!("{}", msg);
}

fn main() {
    let var = 142;

    let res = if var > 100 {
        true
    } else {
        false
    };

    print_result(res);
}
