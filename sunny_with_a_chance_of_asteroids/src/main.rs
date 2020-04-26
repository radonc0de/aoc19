fn main() {
    println!("--Part 1--");
    int_code(1);
    println!("--Part 2--");
    int_code(5);
}

fn int_code(input: i64){
    let mut program = [3,225,1,225,6,6,1100,1,238,225,104,0,1001,152,55,224,1001,224,-68,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,62,41,225,1101,83,71,225,102,59,147,224,101,-944,224,224,4,224,1002,223,8,223,101,3,224,224,1,224,223,223,2,40,139,224,1001,224,-3905,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1101,6,94,224,101,-100,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1102,75,30,225,1102,70,44,224,101,-3080,224,224,4,224,1002,223,8,223,1001,224,4,224,1,223,224,223,1101,55,20,225,1102,55,16,225,1102,13,94,225,1102,16,55,225,1102,13,13,225,1,109,143,224,101,-88,224,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1002,136,57,224,101,-1140,224,224,4,224,1002,223,8,223,101,6,224,224,1,223,224,223,101,76,35,224,1001,224,-138,224,4,224,1002,223,8,223,101,5,224,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,1002,223,2,223,1006,224,329,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,344,101,1,223,223,1107,226,226,224,1002,223,2,223,1006,224,359,1001,223,1,223,1108,677,226,224,102,2,223,223,1005,224,374,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,389,1001,223,1,223,108,677,677,224,1002,223,2,223,1005,224,404,1001,223,1,223,1007,677,677,224,102,2,223,223,1005,224,419,1001,223,1,223,8,226,677,224,102,2,223,223,1005,224,434,101,1,223,223,1008,677,226,224,102,2,223,223,1006,224,449,1001,223,1,223,7,677,677,224,102,2,223,223,1006,224,464,1001,223,1,223,8,226,226,224,1002,223,2,223,1005,224,479,1001,223,1,223,7,226,677,224,102,2,223,223,1006,224,494,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,509,101,1,223,223,107,677,677,224,102,2,223,223,1006,224,524,101,1,223,223,1007,677,226,224,102,2,223,223,1006,224,539,101,1,223,223,107,226,226,224,1002,223,2,223,1006,224,554,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,569,1001,223,1,223,1107,677,226,224,1002,223,2,223,1005,224,584,101,1,223,223,1107,226,677,224,102,2,223,223,1005,224,599,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,614,101,1,223,223,108,677,226,224,102,2,223,223,1005,224,629,101,1,223,223,107,226,677,224,102,2,223,223,1006,224,644,1001,223,1,223,1108,226,226,224,1002,223,2,223,1006,224,659,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,674,101,1,223,223,4,223,99,226];
    let mut i: usize = 0;
    loop{
        //println!("{}", i);
        match program[i]{
            1 => {program[program[i+3] as usize]=add(program[i+1], program[i+2], true, true, &program); i += 4},
            2 => {program[program[i+3] as usize]=multiply(program[i+1], program[i+2], true, true, &program); i += 4},
            3 => {program[program[i+1] as usize] = input; i += 2;}, 
            4 => {println!("Output: {}", program[program[i+1] as usize]); i+=2;},
            5 => i=jump_if_true(i, program[i+1], program[i+2], true, true, &program),
            6 => i=jump_if_false(i, program[i+1], program[i+2], true, true, &program),
            7 => {program[program[i+3] as usize]=less_than(program[i+1], program[i+2], true, true, &program); i+=4},
            8 => {program[program[i+3] as usize]=equal(program[i+1], program[i+2], true, true, &program); i+=4},
            99 => break,
            _ => i=parameter(i, program[i], program[i+1], program[i+2], program[i+3], &mut program)
        }

    }
}

fn add(one: i64, two: i64, position_mode_first: bool, position_mode_second: bool, program: &[i64]) -> i64{
    if position_mode_first && position_mode_second {
        program[one as usize] + program[two as usize]
    }else if position_mode_first{
        program[one as usize] + two
    }else if position_mode_second{
        one + program[two as usize]
    }else{
        one + two
    }
}

fn multiply(one: i64, two: i64, position_mode_first: bool, position_mode_second: bool, program: &[i64]) -> i64{
    if position_mode_first && position_mode_second {
        program[one as usize] * program[two as usize]
    }else if position_mode_first{
        program[one as usize] * two
    }else if position_mode_second{
        one * program[two as usize]
    }else{
        one * two
    }
}

fn less_than(one: i64, two: i64, mode_one: bool, mode_two: bool, program: &[i64]) -> i64{
    if mode_one && mode_two{
        if program[one as usize] < program[two as usize]{
            1
        }else{
            0
        }
    }else if mode_one{
        if program[one as usize] < two {
            1
        }else{
            0
        }
    }else if mode_two{
        if one < program[two as usize] {
            1
        }else{
            0
        }
    }else{
        if one < two {
            1
        }else{
            0
        }
    }
}

fn equal(one: i64, two: i64, mode_one: bool, mode_two: bool, program: &[i64]) -> i64{
    if mode_one && mode_two{
        if program[one as usize] == program[two as usize]{
            1
        }else{
            0
        }
    }else if mode_one{
        if program[one as usize] == two {
            1
        }else{
            0
        }
    }else if mode_two{
        if one == program[two as usize] {
            1
        }else{
            0
        }
    }else{
        if one == two {
            1
        }else{
            0
        }
    }
}

fn jump_if_true(i: usize, one: i64, two: i64, mode_one: bool, mode_two: bool, program: &[i64]) -> usize{
    if mode_one && mode_two{
        if program[one as usize] != 0 {
            program[two as usize] as usize
        }else{
            i+3
        }
    }else if mode_one{
        if program[one as usize] != 0 {
            two as usize
        }else{
            i+3
        }
    }else if mode_two{
        if one != 0 {
            program[two as usize] as usize
        }else{
            i+3
        }
    }else{
        if one != 0{
            two as usize
        }else{
            i+3
        }
    }    
}

fn jump_if_false(i: usize, one: i64, two: i64, mode_one: bool, mode_two: bool, program: &[i64]) -> usize{
    if mode_one && mode_two{
        if program[one as usize] == 0 {
            program[two as usize] as usize
        }else{
            i+3
        }
    }else if mode_one{
        if program[one as usize] == 0 {
            two as usize
        }else{
            i+3
        }
    }else if mode_two{
        if one == 0 {
            program[two as usize] as usize
        }else{
            i+3
        }
    }else{
        if one == 0{
            two as usize 
        }else{
            i+3
        }
    }    
}

fn parameter(i: usize, parameter: i64, one: i64, two: i64, position: i64, program: &mut [i64]) -> usize{
    let nums = num_to_vec(&parameter);
    match nums[2] {
        1 => {
            match nums[1]{
                0 => {
                    match nums[0]{
                        1 => program[position as usize] = add(one, two, true, false, program),
                        _ => program[position as usize] = add(one, two, true, true, program)
                    };
                },1 => {
                    match nums[0]{
                        1 => program[position as usize] = add(one, two, false, false, program),
                        _ => program[position as usize] = add(one, two, false, true, program)
                
                    };  
                },_=> println!("Error, case not being handled")
            }
            i+4
        },2 => {
            match nums[1]{
                0 => {
                    match nums[0]{
                        1 => program[position as usize] = multiply(one, two, true, false, program),
                        _ => program[position as usize] = multiply(one, two, true, true, program)
                    };
                },1 => {
                    match nums[0]{
                        1 => program[position as usize] = multiply(one, two, false, false, program),
                        _ => program[position as usize] = multiply(one, two, false, true, program)
                
                    }; 
                }, _ => println!("Error, case not being handled")
            }
            i+4
        },4 => {
            if nums[1] == 0 {
                println!("Output: {}", program[one as usize]);
            }else if nums[1] == 1 {
                println!("Output: {}", one);
            }
            i+2
            
        },5 => {
            match nums[1]{
                0 => {
                    match nums[0]{
                        1 => jump_if_true(i, one, two, true, false, program),
                        _ => jump_if_true(i, one, two, true, true, program)
                    }
                },1 => {
                    match nums[0]{
                        1 => jump_if_true(i, one, two, false, false, program),
                        _ => jump_if_true(i, one, two, false, true, program)
                    }

                }, _=> {println!("Error, case not being handled"); i}
            }
        },6 => {
            match nums[1]{
                0 => {
                    match nums[0]{
                        1 => jump_if_false(i, one, two, true, false, program),
                        _ => jump_if_false(i, one, two, true, true, program)
                    }
                },1 => {
                    match nums[0]{
                        1 => jump_if_false(i, one, two, false, false, program),
                        _ => jump_if_false(i, one, two, false, true, program)
                    }

                }, _=> {println!("Error, case not being handled"); i}
            }
        },7 => {
            match nums[1]{
                0 => {
                    match nums[0]{
                        1 => program[position as usize] = less_than(one, two, true, false, program),
                        _ => program[position as usize] = less_than(one, two, true, true, program)
                    };
                },1 => {
                    match nums[0]{
                        1 => program[position as usize] = less_than(one, two, false, false, program),
                        _ => program[position as usize] = less_than(one, two, false, true, program)
                
                    };  
                },_=> println!("Error, case not being handled")
            }
            i+4
        },8 => {
            match nums[1]{
                0 => {
                    match nums[0]{
                        1 => program[position as usize] = equal(one, two, true, false, program),
                        _ => program[position as usize] = equal(one, two, true, true, program)
                    };
                },1 => {
                    match nums[0]{
                        1 => program[position as usize] = equal(one, two, false, false, program),
                        _ => program[position as usize] = equal(one, two, false, true, program)
                
                    };  
                },_=> println!("Error, case not being handled")
            }
            i+4
        }_ => {println!("Error, case not being handled"); i}

    }
}


fn num_to_vec(inp: &i64) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();
    
    let mut num = *inp;
    for i in (2..4).rev(){
        let power: i64 = (10_i64).pow(i);
        let val = num / power;
        num = num - (val * power);
        nums.push(val);

    }
    nums.push(num);
    //println!("Parameters split up: {:?}", nums);
    
    nums
}
