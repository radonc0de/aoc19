use std::fs;

fn main() {
    let file = fs::read_to_string("input")
            .expect("Couldn't read file");

    let orbits: Vec<&str> = file.split_whitespace().collect();

    println!("{:?}", orbits);
}
