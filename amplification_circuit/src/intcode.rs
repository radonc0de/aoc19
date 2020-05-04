pub fn int_code(input1: i64, input2: i64) -> i64{
    let mut program = [3,8,1001,8,10,8,105,1,0,0,21,46,55,68,89,110,191,272,353,434,99999,3,9,1002,9,3,9,1001,9,3,9,102,4,9,9,101,4,9,9,1002,9,5,9,4,9,99,3,9,102,3,9,9,4,9,99,3,9,1001,9,5,9,102,4,9,9,4,9,99,3,9,1001,9,5,9,1002,9,2,9,1001,9,5,9,1002,9,3,9,4,9,99,3,9,101,3,9,9,102,3,9,9,101,3,9,9,1002,9,4,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99];
    let mut i: usize = 0;
    let mut second: bool = false;
    let mut output: i64 = 0;
    loop{
	//println!("{}", i);
	match program[i]{
	    1 => {program[program[i+3] as usize]=add(program[i+1], program[i+2], true, true, &program); i += 4},
	    2 => {program[program[i+3] as usize]=multiply(program[i+1], program[i+2], true, true, &program); i += 4},
	    3 => {if second == false{
			program[program[i+1] as usize] = input1; i += 2; second = true;
		}else{
			program[program[i+1] as usize] = input2; i += 2;}
			 
		}, 
	    4 => {output =  program[program[i+1] as usize]; i+=2;},
	    5 => i=jump_if_true(i, program[i+1], program[i+2], true, true, &program),
	    6 => i=jump_if_false(i, program[i+1], program[i+2], true, true, &program),
	    7 => {program[program[i+3] as usize]=less_than(program[i+1], program[i+2], true, true, &program); i+=4},
	    8 => {program[program[i+3] as usize]=equal(program[i+1], program[i+2], true, true, &program); i+=4},
	    99 => break,
	    _ => i=parameter(i, program[i], program[i+1], program[i+2], program[i+3], &mut program)
	}

    }

	output
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
