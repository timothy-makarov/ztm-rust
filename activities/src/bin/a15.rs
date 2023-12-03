// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum TicketType {
    Standard(f32),
    Backstage(f32, String),
    Vip(f32, String)
}

fn main() {
    let tickets = vec![
        TicketType::Standard(10.0),
        TicketType::Backstage(20.0, "Bill".to_owned()),
        TicketType::Vip(40.0, "May".to_owned())
    ];

    for t in tickets {
        match t {
            TicketType::Standard(price) => println!("Standard ticket for {:?}", price),
            TicketType::Backstage(price, name) => println!("Backstage {:?} ticket for {:?}", name, price),
            TicketType::Vip(price, name) => println!("VIP {:?} ticket for {:?}", name, price)
        }
    }
}
