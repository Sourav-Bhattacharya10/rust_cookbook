mod cookbook;

use cookbook::generate_random_number::{generate_random_values, generate_random_values_range};

fn main() {
    // 1.1 Generate Random Values
    generate_random_values();
    line_break();
    generate_random_values_range();
}

fn line_break() {
    println!("\n");
}