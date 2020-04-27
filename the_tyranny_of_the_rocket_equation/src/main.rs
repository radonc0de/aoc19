use std::fs;

fn file_to_vec(file: String) -> Vec<u64>{

    let file_vec: Vec<&str> = file.split_whitespace().collect();
    
    let mut input: Vec<u64> = Vec::new();

    for i in file_vec {
        input.push(i.parse::<u64>().unwrap());    
    }

    input
}

fn main() {
    let file = fs::read_to_string("input")
            .expect("Something went wrong");
    
    let modules = file_to_vec(file);

    println!("Fuel Subtotal : {}", subtotal_fuel(&modules));
    println!("Fuel Total    : {}", total_fuel(&modules));


}

fn subtotal_fuel(modules: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for i in modules{
        sum += (i / 3) - 2
    }

    sum

}

fn total_fuel(modules: &Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for i in modules{
        //println!("Testing module: {}", i);
        let mut val: u64 = *i;
        while ((val / 3)) > 1 {
            val = (val / 3) - 2;
            //println!("New value is: {}", val);
            sum += val;
        }

    }

    sum
}
