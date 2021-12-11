use std::collections::HashMap;
use std::io::BufRead;

fn get_fuel(n: i32) -> i32 {
    n*(n+1)/2
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut crabs: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    crabs.sort();
    let min_number = crabs[0];
    let max_number = crabs[crabs.len() - 1];

    // rust is fast
    let mut min_fuel = i32::MAX;
    let mut old_fuel = 0;
    for i in min_number..max_number+1 {
        let crab_fuel = crabs
            .iter()
            .map(|x| get_fuel((x - i).abs()))
            .sum::<i32>();
	min_fuel = crab_fuel.min(min_fuel);
	if min_fuel == old_fuel {
	    break;
	}	
	old_fuel=crab_fuel;
    }

    //TODO newtons method / binary search for lowest point
    println!("{:?}", min_fuel);
}
