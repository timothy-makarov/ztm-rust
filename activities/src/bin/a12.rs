// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

use std::fmt;

enum BoxColor {
    Red,
    Green,
    Blue,
}

struct Dimensions {
    width: i32,
    height: i32,
    depth: i32
}

impl fmt::Debug for BoxColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxColor::Green => write!(f, "Green"),
            BoxColor::Red => write!(f, "Red"),
            BoxColor::Blue => write!(f, "Blue"),
        }
    }
}

struct ShippingBox {
    dimenstions: Dimensions,
    weight: i32,
    color: BoxColor,
}

impl ShippingBox {
    fn create(width: i32, height: i32, depth: i32, weight: i32, color: BoxColor) -> Self {
        ShippingBox {
            dimenstions: Dimensions { width, height, depth },
            weight,
            color,
        }
    }

    fn print_info(&self) {
        println!(
            "{:?}x{:?}x{:?} {:?} {:?}",
            self.dimenstions.width, self.dimenstions.height, self.dimenstions.depth, self.weight, self.color
        )
    }
}

fn main() {
    let red_box = ShippingBox::create(5, 5, 5, 5, BoxColor::Red);
    red_box.print_info();

    let green_box = ShippingBox::create(50, 50, 50, 50, BoxColor::Green);
    green_box.print_info();

    let blue_box = ShippingBox::create(500, 500, 500, 500, BoxColor::Blue);
    blue_box.print_info();
}
