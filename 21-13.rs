use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut dots: Vec<(i32, i32)> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            break;
        }
        let vs: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        dots.push((vs[0], vs[1]));
    }
    let mut i = 0;
    while let Some(Ok(line)) = lines.next() {
        i += 1;
        let (a, v) = line.split_once('=').unwrap();
        let axis = a.chars().last().unwrap();
        let c: i32 = v.parse().unwrap();
        for i in 0..dots.len() {
            let (x, y) = dots[i];
            let (x2, y2) = match axis {
                'y' if y > c => (x, c * 2 - y),
                'x' if x > c => (c * 2 - x, y),
                _ => (x, y),
            };
            dots[i] = (x2, y2);
        }
        dots.sort();
        dots.dedup();
        println!("{} {} {:?}", i, line, dots.len());
    }
    let (a, b) = dots.iter().last().unwrap();
    for y1 in 0..(*b as i32 + 1) {
        for x1 in 0..(*a as i32 + 1) {
            if dots.contains(&(x1, y1)) {
                print!("\u{2588}");
            } else {
                print!("\u{2593}");
            }
        }
        println!("");
    }
}
