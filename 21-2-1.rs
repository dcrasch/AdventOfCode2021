use std::io::BufRead;

fn main() {
    let stdin= std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;
	
    while let Some(Ok(line)) = lines.next() {
	let (cmd,x) = line.split_once(" ").unwrap();
	let x : i32 = x.parse().unwrap();
	match cmd {
	    "forward" => {
		h+=x;
		d+=x*a;
	    },
	    "down" => {
		a+=x;
	    },
	    "up" => {
		a-=x;
	    },
	    _ =>panic!("oops")
	}
    }
    println!("d={} h={} a={} {}",d,h,a,d*h);
}
