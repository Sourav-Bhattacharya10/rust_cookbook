mod cookbook;

// use cookbook::generate_random_number::{
//     generate_random_value_for_custom_type, generate_random_values, generate_random_values_range,
// };

// use cookbook::sort_vectors::{sort_floats, sort_structs};

// use cookbook::custom_macros;

use cookbook::decompress_zip_files::{compress_into_tar_gz, decompress_tar_xz};

fn main() {
    // println!("oh boy : {}", oh_boy());

    // // 1.1 Generate Random Values
    // generate_random_values();
    // line_break();
    // generate_random_values_range();
    // line_break();
    // generate_random_value_for_custom_type();

    // // 1.2 Sort Vecctors
    // sort_floats();
    // line_break();
    // sort_structs();

    // // // 1.3 Declarative Custom Macros
    // let temp_list = list![10; 5];
    // // let temp_list = vec![1, 2, 3];
    // println!("{:?}", temp_list);

    // 3.1 Decompress Zip Files
    let file_path = "/home/souravbhattacharya/Documents/Visual_Studio_Code/Rust_Projects/rust_cookbook/samples.tar.xz";
    match decompress_tar_xz(file_path) {
        Ok(_) => println!("Extracted {file_path} successfully"),
        Err(err) => eprintln!("Error occurred while extracting {file_path} : {err}"),
    };

    // 3.2 Compress Files
    let gz_file_name = "ferris.tar.gz";
    let file_path = "./Ferris.png";
    match compress_into_tar_gz(file_path, gz_file_name) {
        Ok(_) => println!("Compressed {gz_file_name} successfully"),
        Err(err) => eprintln!("Error occurred while compressing {gz_file_name} : {err}"),
    };
}

// // hoisting with const
// fn oh_boy() -> u8 {
//     return X;

//     const X: u8 = 10;
// }

// fn line_break() {
//     println!("\n");
// }
