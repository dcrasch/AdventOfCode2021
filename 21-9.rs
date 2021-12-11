use std::io::{BufRead};
use std::collections::VecDeque;
use std::cmp::Reverse;

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    let mut grid : Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
	let row = line.chars()
	    .map(|x|x as u8 - b'0')
	    .collect();
	grid.push(row);
    }
    let width = grid[0].len();
    let height = grid.len();


    let mut risk_level = 0_i32;
    let mut low_points : Vec<(usize,usize)> = Vec::new();
    for row in 0..height {
	for col in 0..width {
	    let point = grid[row][col];
	    if point==9 {
		continue;
	    }
	    let mut low = true;
	    if row>0 && point>=grid[row-1][col] {
		low = false;
	    }
	    if col>0 && point>=grid[row][col-1] {
		low = false;
	    }
	    if row<height-1 && point>=grid[row+1][col] {
		low = false;
	    }
	    if col<width-1 && point>=grid[row][col+1] {
		low = false;
	    }
	    if low {
		low_points.push((row,col));
		risk_level+= point as i32 +1;
	    }
	}
    }    
    println!("{}",risk_level);
    let mut basins : Vec<usize> = Vec::new();
    for &(rr,cc) in low_points.iter() {
	let p = grid[rr][cc];
	// floodfill
	let mut q : VecDeque<(usize,usize)> = VecDeque::new();
	q.push_back((rr,cc));
	let mut basin_size = 0;
	while !q.is_empty() {
	    let (row,col) = q.pop_front().unwrap();
	    let point = grid[row][col];
	    if point==9 {
		continue;
	    }
	    basin_size+=1;
	    grid[row][col]=9;
	    if row>0 {
		q.push_back((row-1,col));
	    }
	    if col>0  {
		q.push_back((row,col-1));
	    }
	    if row<height-1 {
		q.push_back((row+1,col));
	    }
	    if col<width-1  {
		q.push_back((row,col+1));
	    }
	}
	basins.push(basin_size);
    }
    println!("{:?}",basins);
    basins.sort_by_key(|w| Reverse(*w));
    let sol = basins[0]*basins[1]*basins[2];
    println!("{} * {} * {} = {}",basins[0],basins[1],basins[2],sol);
}
	    
	
