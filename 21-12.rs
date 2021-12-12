use std::io::{BufRead};
use std::collections::{HashMap,HashSet};

#[derive(Debug)]
struct Padje<'a> {
    visited : HashSet<&'a str>,
    twice : bool
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        let mut vs = line.split('-');
        let a = vs.next().unwrap();
        let b = vs.next().unwrap();
	if b!="start" && a!="end" {
	    (*m.entry(a.to_string()).or_insert(Vec::new())).push(b.to_string());
	}
	if a!="start" && b!="end" {
            (*m.entry(b.to_string()).or_insert(Vec::new())).push(a.to_string());
	}
    }
    let mut stack: Vec<(&str,Padje)> = Vec::new();
    stack.push(("start",Padje {visited:HashSet::new(), twice:false}));
    let mut count = 0;
    while let Some((c,padje)) = stack.pop() {
        if let Some(caves) = m.get(c) {
            for cave in caves.iter() {
		if cave=="end" {
		    //println!("{:?}",padje.visited);
		    count+=1;
		}
		else {
		    let firstletter : char = cave.chars().nth(0).unwrap();
		    let mut twice = padje.twice;
		    let mut p = padje.visited.clone();
		    if firstletter.is_ascii_lowercase() {
			if padje.visited.contains(cave.as_str()) {
			    if padje.twice {
				continue;
			    }
			    twice = true;			    
			}			
			p.insert(cave);
		    }
                    stack.push((cave,Padje {visited:p, twice:twice}));
                }
            }
        }
    }
    println!("sol1 {}",count);
}
	
