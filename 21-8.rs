use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn decode_len(numbers: &HashSet<char>) -> Option<u32> {
    match numbers.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

/*
0=6
1=2
2=5
3=5
4=4
5=5
6=6
7=3
8=7
9=6
 */

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut count = 0;
    let mut total_sum = 0;
    while let Some(Ok(line)) = lines.next() {
        let (p1, p2) = line.split_once('|').unwrap();
        let signals: Vec<HashSet<char>> = p1
            .trim()
            .split(' ')
            .map(|x| x.chars().collect::<HashSet<char>>())
            .collect();
        let output: Vec<&str> = p2
	    .trim()
	    .split(' ')

	    .collect();

        let mut m: HashMap<u32, &HashSet<char>> = HashMap::new();

        // 1,4,7,8
        for signal in signals.iter() {
            if let Some(n) = decode_len(signal) {
                m.insert(n, &signal);
            }
        }
        // 6,9,0
        for signal in signals.iter() {
            if signal.len() == 6 {
                if signal.is_superset(m.get(&1).unwrap()) {
                    if signal.is_superset(m.get(&4).unwrap()) {
                        m.insert(9, &signal);
                    } else {
                        m.insert(0, &signal);
                    }
                } else {
                    m.insert(6, &signal);
                }
            }
        }
        // 2,3,5
        for signal in signals.iter() {
            if signal.len() == 5 {
                if signal.is_superset(m.get(&1).unwrap()) {
                    m.insert(3, &signal);
                } else {
                    if signal.intersection(m.get(&9).unwrap()).count() == 4 {
                        m.insert(2, &signal);
                    } else {
                        m.insert(5, &signal);
                    }
                }
            }
        }

        let m: HashMap<String, u32> = m
            .iter()
            .map(|(k, v)| {
                let mut v: Vec<char> = v.iter().map(|x| x.clone()).collect();
                v.sort();
                let a: String = v.iter().collect();
                (a, *k)
            })
            .collect();
        println!("{:?}", signals);
        println!("{:?}", output);
        println!("{:?}", m);
        let mut res = 0;
        for a in output.iter() {
            let mut b: Vec<char> = a.chars().collect();
            b.sort();
            let b: String = b.iter().collect();
            res = res * 10;
            let d = m.get(&b).unwrap();
            res += d;
        }
        total_sum += res;
        println!("res={}", res);
    }
    println!("{} {}", count, total_sum);
}
