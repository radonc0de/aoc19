fn main() {

    let passwords = find_passwords();
    println!("There are {} passwords.", passwords.len());
    
    let num_working_passwords = find_working_passwords(passwords).len();
    println!("There are {} passwords now.", num_working_passwords);

}

fn find_passwords() -> Vec<u32> {
    let mut passwords: Vec<u32> = Vec::new();

    for i in 240298..784956 {
        let mut works = false;
        println!("Checking {}", i);
        let nums: Vec<u32> = nums(&i);
        if nums[0] == nums[1] || nums[1] == nums[2] || nums[2] == nums[3] || nums[3] == nums[4] || nums[4] == nums[5]{
            println!("Passed the first test");
            works = true;
            for j in 1..6{
                if nums[j] < nums[j-1] {
                    works = false;
                }  
            }
        }

        if works {
            println!("Passed the second test");
            passwords.push(i);
        }
    }

    passwords
}
    
fn find_working_passwords(passwords: Vec<u32>) -> Vec<u32> {

    let mut working_passwords: Vec<u32> = Vec::new();

    for i in passwords {
        let nums = nums(&i);
        let mut works = false;
        if nums[0] == nums[1] && nums[0] != nums[2]{
            works = true;
        }else if nums[1] == nums[2] && nums[1] != nums[0] && nums[1] != nums[3] {
            works = true;
        }else if nums[2] == nums[3] && nums[2] != nums[1] && nums[2] != nums[4] {
            works = true;
        }else if nums[3] == nums[4] && nums[3] != nums[2] && nums[3] != nums[5] {
            works = true;
        }else if nums[4] == nums[5] && nums[4] != nums[3]{
            works = true;
        }
        
        if works {
            working_passwords.push(i);
        }

    }

    working_passwords

}

fn nums(inp: &u32) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    
    let mut num = *inp;
    for i in (0..6).rev(){
        let power: u32 = (10_u32).pow(i);
        let val = num / power;
        num = num - (val * power);
        nums.push(val);

    }
    nums
}
