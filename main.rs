use std::io::BufRead;
use std::collections::{HashMap,HashSet};

fn main() {
    let stdin = std::io::stdin();
    let lines : Vec<String> = stdin.lock().lines()
	.map(|x|x.unwrap())
	.collect();
    let missing_parts : u64 = lines[0].split_once(" ").unwrap().0.parse().unwrap();
    let mut workshop : HashMap<&str,Vec<(&str,u64)>> = HashMap::new();
    for line in &lines[1..] {
        let (part_name,subparts_str) = line.split_once(":").unwrap();
        let mut subparts : Vec<(&str,u64)> = Vec::new();
        for subpart in subparts_str.split(",") {
            let (amount,subpart_name) = subpart.trim().split_once(" ").unwrap();
            let	amount :u64 = amount.parse().unwrap();
            subparts.push((subpart_name,amount))
	}
	workshop.insert(part_name,subparts);
    }

    let mut cost : HashMap<&str, u64> = HashMap::new();
    for p in workshop.keys() {
	let mut stack : Vec<(&str,u64)> = Vec::new();
	let mut total_cost = 0;
	stack.push((&p,1));
	while let Some((part,amount)) = stack.pop()  {
	    if let Some(subparts) = workshop.get(part) {
		for (subpart,subpart_amount) in subparts {		    
		    stack.push((subpart,*subpart_amount*amount));
		}
	    }
	    else {
		total_cost+=1*amount;
	    }
	}
	cost.insert(p,total_cost);
    }
    
    println!("solution 1 = {:#?}",cost.values().max().expect("no solution 1"));

    let mut presents : HashSet<&str> = workshop.keys().map(|x|*x).collect();
    for part in workshop.keys() {
	if let Some(subparts) = workshop.get(part) {
	    for (subpart,_) in subparts {
		presents.remove(subpart);
	    }
	}
    }

    println!("missing parts = {}",missing_parts);
    let mut numbers5 : Vec<u64> = cost.iter().filter(|(x,_)|presents.contains(*x)).map(|(_,y)|y.clone()).collect();
    numbers5.sort();
    let mut subset5 : HashSet<u64> = HashSet::new();
    for a in &numbers5 {
	for b in &numbers5 {
	    for c in &numbers5 {
		for d in &numbers5 {
		    for e in &numbers5 {
			let s = a+b+c+d+e;
			if s<missing_parts {
			    subset5.insert(s);
			}
		    }
		}
	    }
	}
    }
    
    println!("5 sets: {}",subset5.len());
    let mut numbers10 : Vec<&u64> = subset5.iter().collect();
    numbers10.sort();
    let mut subset10 : HashSet<u64> = HashSet::new();
    for a in &numbers10 {
	for b in &numbers10 {
	    let s = *a+*b;
	    if s<missing_parts {
		subset10.insert(s);
	    }
	}
    }
    
    println!("10 sets: {}",subset10.len());
    let mut numbers20  : Vec<u64> = subset10.iter().map(|x|x.clone()).collect();
    numbers20.sort();
    let mut res2 = [0,0];
    for a in numbers20.iter() {
	let b = &missing_parts-*a;
	if let Ok(_) = numbers20.binary_search(&b) {
	    let p1=a.clone();
	    let p2=b.clone();
	    if p1+p2==missing_parts {
		res2[0]=a.clone();
		res2[1]=b.clone();
		break;
	    }
	}
    }
    println!("res2 {:?} {}",res2,res2.iter().sum::<u64>());

    let mut res4 = [0;4];
    for (i,p) in res2.iter().enumerate() {
	for a in &numbers10 {
	    for b in &numbers10 {
		let s = a.clone()+b.clone();
		if s==*p {
		    res4[i*2]=**a;
		    res4[i*2+1]=**b;
		}
	    }
	}
    }

    println!("res4 {:?} {}",res4,res4.iter().sum::<u64>());
    
    let mut res20 = [0;20];
    for (i,p) in res4.iter().enumerate() {
	for a in &numbers5 {
	    for b in &numbers5 {
		for c in &numbers5 {
		    for d in &numbers5 {
			for e in &numbers5 {
			    let s = a+b+c+d+e;
			    if *p==s {
				res20[i*5+0]=*a;
				res20[i*5+1]=*b;
				res20[i*5+2]=*c;
				res20[i*5+3]=*d;
				res20[i*5+4]=*e;
			    }
			}
		    }
		}
	    }
	}
    }
    println!("res20 {:?}",res20);

    let rc : HashMap<u64,&str> = cost.iter().map(|(x,y)|(y.clone(),*x)).collect();
    let mut res : Vec<&str> = Vec::new();
    for r in res20.iter() {
	res.push(rc.get(r).unwrap_or(&""));
    }
    res.sort();
    println!("{:?}",res);
    println!("solution = {}",res.iter().map(|x|x.chars().nth(0).unwrap()).collect::<String>());
						
}

