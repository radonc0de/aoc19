use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle_input")
            .expect("Couldn't read file");

    let orbits: Vec<&str> = file.split_whitespace().collect();

    println!("{:?}", orbits);

    println!("Total orbits = {}", ind_orbits(orbits));
}

fn ind_orbits(mut orbits: Vec<&str>) -> u64{
    let mut all_orbits: Vec<Vec<&str>> = Vec::new();
    let mut COM_val: usize = 0;
    let mut COM_index: usize = 0;
    for i in &orbits{
        let test_val = &i[0..3];
        if test_val == "COM" {
            println!("Match found. It is {}. Adding to vector.", &i[4..]);
            all_orbits = vec![vec![&i[4..]]];
            println!("The value was succesfully added to the vector!");
            COM_index = COM_val;
        }
        COM_val += 1;
    }  
    orbits.remove(COM_index);
    
    while orbits.len() > 0 {
        let mut indexes_to_remove: Vec<usize> = Vec::new();
        let mut cur_index = 0;
        for i in &orbits{
            let length = all_orbits.len();
            let index = all_orbits.len();
            for j in 0.. index{
                let last_index: usize = all_orbits[j].len() - 1;
                let test_vec = &all_orbits[j];
                let test_str = test_vec[last_index];

                if &i[0..3] == test_str{
                    all_orbits.push(all_orbits[j].to_vec());
                    //println!("ALL ORBITS: {:?}", all_orbits);
                    println!("Pushing {} to all_orbits[{}]", &i[4..], length);
                    all_orbits[length].push(&i[4..]);
                    indexes_to_remove.push(cur_index);
                }
            }
            cur_index += 1;
        }
        
        let mut fix: usize = 0;
        for i in indexes_to_remove {
            orbits.remove(i - fix);
            fix += 1;
        }
    }
   
    let mut sum: u64 = 0;

    for k in all_orbits{
        sum += k.len() as u64;
    }

    sum
}

