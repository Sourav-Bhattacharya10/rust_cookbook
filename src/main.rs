mod cookbook;

// use cookbook::generate_random_number::{
//     generate_random_value_for_custom_type, generate_random_values, generate_random_values_range,
// };

use cookbook::sort_vectors::{sort_floats, sort_structs};

fn main() {
    // println!("oh boy : {}", oh_boy());

    // // 1.1 Generate Random Values
    // generate_random_values();
    // line_break();
    // generate_random_values_range();
    // line_break();
    // generate_random_value_for_custom_type();

    // // 1.2 Sort Vecctors
    sort_floats();
    line_break();
    sort_structs();
}

// // hoisting with const
// fn oh_boy() -> u8 {
//     return X;

//     const X: u8 = 10;
// }

fn line_break() {
    println!("\n");
}
