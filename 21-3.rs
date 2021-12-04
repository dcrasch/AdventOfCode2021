use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut rows: Vec<Vec<bool>> = lines
        .map(|line| {
            line.unwrap()
                .bytes()
                .map(|x| x == b'1')
                .collect::<Vec<bool>>()
        })
        .collect();
    let k = rows[0].len();
    rows.sort();
    let mut cols : Vec<usize> = Vec::new();
    // solution1
    let mut gamma = 0_u64;
    let mut epsilon = 0_u64;
    for i in 0..k {
        let ones = rows.iter().filter(|vs| vs[i]).count();
        let zeroes = rows.len() - ones;
        gamma <<= 1;
        epsilon <<= 1;
        if ones > zeroes {
            gamma += 1;
        } else {
            epsilon += 1;
        }
	cols.push(ones);
    }
    println!("sol1: {} x {} = {}", gamma, epsilon, epsilon * gamma);


    for r in rows.iter() {
	for i in 0..k {
	    print!("{}",if r[i] { "1" } else {"0"});
	}
	println!("");
    }
	

    // solution2
    let mut oxygen = rows.clone();
    for i in 0..k {
        if oxygen.len() == 1 {
            break;
        }
        let ones = oxygen.iter().filter(|vs| vs[i]).count();
        let zeroes = oxygen.len() - ones;
        let common = ones >= zeroes;
        oxygen.retain(|v| v[i] == common);
    }
    let oxygen = oxygen[0].clone();
    let oxygen = oxygen
        .iter()
        .map(|&x| if x { '1' } else { '0' })
        .collect::<String>();
    let oxygen = u128::from_str_radix(&oxygen, 2).unwrap();

    let mut co2 = rows.clone();
    for i in 0..k {
        if co2.len() == 1 {
            break;
        }
        let ones = co2.iter().filter(|vs| vs[i]).count();
        let zeroes = co2.len() - ones;
        let least_common = ones < zeroes;
        co2.retain(|v| v[i] == least_common);
    }
    let co2 = co2[0].clone();
    let co2 = co2
        .iter()
        .map(|&x| if x { '1' } else { '0' })
        .collect::<String>();
    let co2 = u128::from_str_radix(&co2, 2).unwrap();

    println!("sol2: {} x {} = {}", oxygen, co2, oxygen * co2)
}
