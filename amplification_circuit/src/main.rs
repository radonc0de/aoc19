mod intcode;

fn main() {
    println!("Best signal is {}", find_best());
}

fn get_signal(a: i64, b:i64, c:i64, d:i64, e:i64) -> i64{
    intcode::int_code(e, intcode::int_code(d, intcode::int_code(c, intcode::int_code(b, intcode::int_code(a, 0)))))
}

fn find_best() -> i64 {
    let mut signals: Vec<i64> = Vec::new();
    let combinations: Vec<i64> = vec![01234,01243,01324,01342,01423,01432,02134,02143,02314,02341,02413,02431,03124,03142,03214,03241,03421,03412,04123,04132,04213,04231,04321,04312,10234,10243,10432,10423,10324,10342,12034,12043,12304,12340,12430,12403,13024,13042,13204,13240,13420,13402,14023,14032,14203,14230,14302,14320,20134,20143,20314,20341,20413,20431,21034,21043,21304,21340,21430,21403,23014,23041,23104,23140,23410,23401,24013,24031,24103,24130,24310,24301,30124,30142,30241,30214,30412,30421,31024,31042,31240,31204,31420,31402,32014,32041,32104,32140,32401,32410,34012,34021,34102,34120,34201,34210,40123,40132,40213,40231,40321,40312,41023,41032,41203,41230,41302,41320,42013,42031,42103,42130,42310,42301,43012,43021,43102,43120,43210,43201];
    for i in combinations {
        let test_combos = num_to_vec(i);
        let signal = get_signal(test_combos[0], test_combos[1], test_combos[2], test_combos[3], test_combos[4]);
	//println!("Running with {}, {}, {}, {}, {} resulted in {}", test_combos[0], test_combos[1], test_combos[2], test_combos[3], test_combos[4], signal);	
	signals.push(signal);
    }
    signals.sort();
    signals[signals.len() - 1]
}

fn num_to_vec(num: i64) -> Vec<i64> {
    let mut x = num;
    let mut vec: Vec<i64> = Vec::new();
    
    let mut val: i64 = x / 10000;
    vec.push(val);
    x = x - (10000 * val);
    val = x / 1000;
    vec.push(val);
    x = x - (1000 * val);
    val = x / 100;
    vec.push(val);
    x = x - (100 * val);
    val  = x / 10;
    vec.push(val);
    x = x - (10 * val);
    vec.push(x);

    vec
}
