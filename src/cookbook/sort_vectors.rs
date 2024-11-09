pub fn sort_floats() {
    let mut vec_floats = vec![1.9, 1.3, 1.7, 1.2];
    vec_floats.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("sort floats : {:?}", vec_floats);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Person {
    pub name: String,
    pub age: u8,
}

pub fn sort_structs() {
    let mut vec_persons = vec![
        Person {
            name: String::from("Beta"),
            age: 3,
        },
        Person {
            name: String::from("Beta"),
            age: 2,
        },
        Person {
            name: String::from("Charlie"),
            age: 4,
        },
        Person {
            name: String::from("Alfa"),
            age: 1,
        },
        Person {
            name: String::from("Delta"),
            age: 5,
        },
    ];

    vec_persons.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("sort floats : {:?}", vec_persons);
}
