use std::io::{BufRead};
use std::collections::{HashMap};

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let polymer_template = lines.next().unwrap().unwrap().to_string();
    let _ = lines.next();

    let mut rules : HashMap<String,String> = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
	let (p1,p2) = line.split_once(" -> ").unwrap();
	rules.insert(p1.to_string(),p2.to_string());
    }

    println!("polymer template {}",polymer_template);
    println!("rules {:?}",rules);

    //part1 brute force
    let mut freqs : HashMap<char,usize> = HashMap::new();
    let mut polymer = polymer_template.to_string();
    for c in polymer.chars() {
	*freqs.entry(c).or_insert(0)+=1;
    }
    for _ in 0..10 {
	let mut r = "".to_string();
	let k = polymer.len();
	for j in 0..k-1 {
	    let a = &polymer[j..j+2];
	    r.push_str(&polymer[j..j+1]);
	    if let Some(b) = rules.get(a) {
		r.push_str(b);
		*freqs.entry(b.chars().next().unwrap()).or_insert(0)+=1;
	    }
	}
	r.push_str(&polymer[k-1..k]);
	polymer = r;
    }
    let min = freqs.values().min().unwrap();
    let max = freqs.values().max().unwrap();
    println!("{}-{} = {}",min,max,max-min);

    println!("---------");
    //part2
    let mut freqs : HashMap<char,usize> = HashMap::new();
    let mut pairs : HashMap<String,usize> = HashMap::new();
    for i in 0..polymer_template.len()-1 {
	let cc = &polymer_template[i..i+2];
	let first = cc.chars().next().unwrap();
	*pairs.entry(cc.to_string()).or_insert(0)+=1;
	*freqs.entry(first).or_insert(0)+=1;
    }
    let last = polymer_template.chars().last().unwrap();
    *freqs.entry(last).or_insert(0)+=1;
    
    for _ in 0..40 {
	let mut new_pairs : HashMap<String,usize> = HashMap::new();
	for (cc,&f) in pairs.iter() {
	    if let Some(b) = rules.get(&cc.to_string()) {
		let left = format!("{}{}",cc[0..1].to_string(),b);
		let right = format!("{}{}",b,cc[1..2].to_string());
		*new_pairs.entry(left).or_insert(0)+=f;
		*new_pairs.entry(right).or_insert(0)+=f;
		*freqs.entry(b.chars().next().unwrap()).or_insert(0)+=f;
	    }
	}
	pairs = new_pairs;
    }
    let min = freqs.values().min().unwrap();
    let max = freqs.values().max().unwrap();
    println!("{}-{} = {}",min,max,max-min);
}
