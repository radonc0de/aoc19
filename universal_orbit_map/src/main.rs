use std::fs;

fn main() {
    let file = fs::read_to_string("input")
            .expect("Couldn't read file");

    let orbits: Vec<&str> = file.split_whitespace().collect();

    println!("{:?}", orbits);

    println!("Total orbits = {}", ind_orbits(orbits));
}

fn ind_orbits(mut orbits: Vec<&str>) -> u64{
    let mut all_orbits: Vec<Vec<&str>> = Vec::new();
    //let mut all_orbits: Vec<Vec&str>> = Vec:new();
    for i in &orbits{
        println!("Determing what value is attached to COM");
        let test_val = &i[0..3];
        println!("Testing {}", test_val);
        println!("Against {}", "COM");
        if test_val == "COM" {
            println!("Match found. It is {}. Adding to vector.", &i[4..]);
            all_orbits = vec![vec![&i[4..]]];
            println!("The value was succesfully added to the vector!");
        }
    }  

    orbits.remove(0);
    //println!("{:?}", all_orbits);
    
    for i in &orbits{
        println!("Testing orbit {}", i);
        let length = all_orbits.len();
        let index = all_orbits.len();
        println!("Index = {}", index);
        for j in 0.. index{
            println!("About to figure out location. On index {}", j);
            let last_index: usize = all_orbits[j].len() - 1;
            let test_vec = &all_orbits[j];
            let test_str = test_vec[last_index];

            all_orbits[0][0] = &i[4..];
            println!("Testing {} against {}", &i[0..3], test_str);

            if &i[0..3] == test_str{
                println!("Found it!. Copying all_orbits[{}] to all_orbits[{}]", j, length);
                all_orbits[length] = vec_clone(&all_orbits[j]);
                println!("All Orbits: {:?}", all_orbits);
                all_orbits[length].push(&i[5..8]);
            }
        }
        println!("{:?}", all_orbits);
    }

    let mut sum: u64 = 0;

    for k in all_orbits{
        sum += k.len() as u64 - 1
    }

    sum
}

fn vec_clone(cloning: &Vec<&str>) -> Vec<&str> {
    *cloning
}
