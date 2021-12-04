use std::{io::BufRead};
use std::collections::HashSet;

fn check_row(r: &[u32], numbers:&HashSet<u32>) -> bool {
    r.iter().all(|x|numbers.contains(x))
}
fn check_board(b: &Vec<Vec<u32>>, numbers:&HashSet<u32>) -> bool {
    let mut f= b.iter().any(|r|check_row(r,numbers));

    for i in 0..5 {
        f= f || check_row(&vec![b[0][i],b[1][i],b[2][i],b[3][i],b[4][i]],numbers);
    }
    f
}
fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let numbers : Vec<u32> = lines.next().unwrap().unwrap()
        .split(',')
        .map(|x|x.parse().unwrap())
        .collect();

    //println!("{:?}",numbers);
    let mut boards : Vec<Vec<Vec<u32>>> = Vec::new();
    while let Some(Ok(_)) = lines.next() {
        let mut b : Vec<Vec<u32>> = Vec::new();
        for _ in 0..5 {
            let a= lines.next().unwrap().unwrap()
                .split(" ")
                .flat_map(|x|x.parse())
                    .collect::<Vec<u32>>();
            b.push(a);
	}
        boards.push(b);        
    }
    let mut drawn : HashSet<u32> = HashSet::new();
    let mut wins : Vec<usize> = Vec::new();
    let mut lastnumber : Vec<u32> = Vec::new();
    let mut lastsum : Vec<u32> = Vec::new();

    for n in numbers {
        drawn.insert(n);
        for (i,b) in boards.iter().enumerate() {
            if check_board(b, &drawn) {
                if !wins.contains(&i) {
                    wins.push(i);
                    lastnumber.push(n);
                    let bn  = b.iter().flatten().map(|x|x.clone()).collect::<HashSet<u32>>();
                    let d : u32 = bn.difference(&drawn).sum();
                    lastsum.push(d);
                }
            }
        }
    }
    let n = lastnumber.last().unwrap();
    let i = wins.last().unwrap();
    let d =lastsum.last().unwrap();
    println!("{} {} = {}",d,n,d*n);
}
