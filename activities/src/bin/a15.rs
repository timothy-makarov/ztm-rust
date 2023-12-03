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
    Standard,
    Backstage(String),
    Vip(String)
}

struct Ticket {
    ticket_type: TicketType,
    price: f32
}

fn main() {
    let tickets = vec![
        Ticket {
            ticket_type: TicketType::Standard,
            price: 5.0
        },
        Ticket {
            ticket_type: TicketType::Vip("Sarah".to_owned()),
            price: 10.0
        },
        Ticket {
            ticket_type: TicketType::Backstage("Jessie".to_owned()),
            price: 20.0
        }
    ];

    for t in tickets {
        match t.ticket_type {
            TicketType::Standard => println!("Standard ticket for {:?}", t.price),
            TicketType::Vip(name) => println!("{:?} VIP ticket for {:?}", name, t.price),
            TicketType::Backstage(name) => println!("{:?} Backstage ticket for {:?}", name, t.price)
        }
    }
}
