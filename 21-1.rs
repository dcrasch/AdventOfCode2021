use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines();
    let numbers : Vec<i32> = lines.map(|x|x.unwrap().parse().unwrap()).collect();
    let numbers : Vec<i32> = numbers.windows(3).map(|a|a.iter().sum()).collect();
    let c = numbers.windows(2).filter(|a|a[0]<a[1]).count();
    println!("{}",c);
}
