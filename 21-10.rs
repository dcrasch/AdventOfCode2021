use std::io::{BufRead};

fn matchpair(c:char) -> char {
    match c {
	'(' => ')',
	'[' => ']',
	'{' => '}',
	'<' => '>',
	')' => '(',
	']' => '[',
	'}' => '{',
	'>' => '<',
	_ => panic!("oops")
    }
}

fn score_incomplete(c:char) -> u128 {
    match c {
	')' => 1,
	']' => 2,
	'}' => 3,
	'>' => 4,
	_ => 0
    }
}

fn score_illegal(c:char) -> u128 {
    match c {
	')' => 3,
	']' => 57,
	'}' => 1197,
	'>' => 25137,
	_ => 0
    }
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let mut total_illegal_score = 0_u128;
    let mut incomplete_scores : Vec<u128> = Vec::new();
    
    while let Some(Ok(line))=lines.next() {
	let mut stack : Vec<char> = Vec::new();
	let mut illegal_score = 0;
	for c in line.chars() {
	    match c {
		'('|'['|'{'|'<' => stack.push(c),
		')'|']'|'}'|'>' => {
		    match (stack.pop(),c) {
			(Some('('),')')|
			(Some('['),']')|
			(Some('{'),'}')|
			(Some('<'),'>') => (),
			(Some(x),y) => {
			    eprintln!("Expected {}, but found {} instead",matchpair(x),y);
			    illegal_score=score_illegal(y);
			    break;
			},	
			_ => panic!("oops")
		    }
		}
		_ => panic!("onbekend char {}",c)
	    }
	}
	if illegal_score !=0 {
	    println!("score illegal {}",illegal_score);
	    total_illegal_score = total_illegal_score + illegal_score;
	}
	else {
	    if stack.len()!=0 {
		eprint!("Complete by adding");
		let mut incomplete_score = 0_u128;
		for &c in stack.iter().rev() {
		    let m = matchpair(c);
		    eprint!("{}",m);
		    incomplete_score = incomplete_score * 5 + score_incomplete(m);
		}
		eprintln!(".");
		println!("{}",incomplete_score);
		incomplete_scores.push(incomplete_score);
	    }
	}
    }
    println!("illegal {}",total_illegal_score);

    incomplete_scores.sort();
    let middle_score = incomplete_scores[incomplete_scores.len()/2];
    println!("incomplete middle score:{}",middle_score);
}
	
