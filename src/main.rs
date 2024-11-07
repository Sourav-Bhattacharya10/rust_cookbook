mod cookbook;

use cookbook::generate_random_number::{
    generate_random_value_for_custom_type, generate_random_values, generate_random_values_range,
};

fn main() {
    // 1.1 Generate Random Values
    generate_random_values();
    line_break();
    generate_random_values_range();
    line_break();
    generate_random_value_for_custom_type();
}

fn line_break() {
    println!("\n");
}
