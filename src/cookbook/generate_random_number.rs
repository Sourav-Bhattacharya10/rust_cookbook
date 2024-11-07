// 1.1 Generate Random Values

use rand::Rng;

pub fn generate_random_values() {
    let mut rng =  rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2 : u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

pub fn generate_random_values_range() {
    let mut rng = rand::thread_rng();
    
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}