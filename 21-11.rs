use std::io::{self, BufRead};

fn read_grid<T: BufRead>(c: T) -> Vec<Vec<u8>> {
    let mut lines = c.lines();
    let mut grid: Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let row: Vec<u8> = line.bytes().map(|x| x - b'0').collect();
        grid.push(row);
    }
    grid
}

fn dirs(r: usize, c: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut ds: Vec<(usize, usize)> = Vec::new();
    if r > 0 {
        ds.push((r - 1, c));
        if c > 0 {
            ds.push((r - 1, c - 1));
        }
        if c < w - 1 {
            ds.push((r - 1, c + 1));
        }
    }
    if c > 0 {
        ds.push((r, c - 1));
    }
    if c < w - 1 {
        ds.push((r, c + 1));
    }
    if r < h - 1 {
        ds.push((r + 1, c));
        if c > 0 {
            ds.push((r + 1, c - 1));
        }
        if c < w - 1 {
            ds.push((r + 1, c + 1));
        }
    }
    ds
}

fn step(grid: &mut Vec<Vec<u8>>) -> u32 {
    let h = grid.len();
    let w = grid[0].len();
    let mut flashed=0;
    //1
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for row in 0..h {
        for col in 0..w {
            grid[row][col] += 1;
            if grid[row][col] > 9 {
                stack.push((row, col));
            }
        }
    }

    while let Some(octo) = stack.pop() {
        let (row, col) = octo;
        let flashes = grid[row][col];
        if flashes == 0 {
            continue;
        }
	flashed+=1;
        grid[row][col] = 0;
        for (r, c) in dirs(row, col, w, h) {
	    if grid[r][c]!=0 {
		grid[r][c] += 1;
		if grid[r][c] > 9 {
                    stack.push((r, c));
		}
	    }
        }
    }
    flashed
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut grid = read_grid(reader);
    let mut flashes = 0;
    let zeroes : Vec<Vec<u8>> = vec![vec![0;grid[0].len()];grid.len()];

/*    for i in 0..100 {
	let res = step(&mut grid);
	flashes+=res;
    }
    println!("{}",flashes);
     */
    let mut step_count=0;
    while grid!=zeroes {
	let res = step(&mut grid);
	step_count+=1;
    }
    println!("{}",step_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1() {
        let cursor = io::Cursor::new(
            b"11111
19991
19191
19991
11111",
        );
        let mut g1 = read_grid(cursor);
        let cursor = io::Cursor::new(
            b"34543
40004
50005
40004
34543",
        );
        let mut g2 = read_grid(cursor);
        let res = step(&mut g1);
        assert_eq!(g2, g1);
	assert_eq!(9,res);
    }

    #[test]
    fn step2() {
        let cursor = io::Cursor::new(
            b"34543
40004
50005
40004
34543",
        );
        let mut g1 = read_grid(cursor);
        let cursor = io::Cursor::new(
            b"45654
51115
61116
51115
45654");
	
        let mut g2 = read_grid(cursor);

        let res = step(&mut g1);
        assert_eq!(g2, g1);
	assert_eq!(res,0);
    }
    
    #[test]
    fn exploration() {
        let cursor = io::Cursor::new(
            b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
        );

        assert_eq!(
            read_grid(cursor),
            vec![
                vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6]
            ]
        );
    }

    #[test]
    fn steps10() {
	let cursor = io::Cursor::new(b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526");
	
	let mut g1 = read_grid(cursor);
	let mut total_flashes = 0;
	for _ in 0..10 {
            let res = step(&mut g1);
	    total_flashes+=res;
	}
	assert_eq!(total_flashes,204);
    }

    #[test]
    fn steps100() {
	let cursor = io::Cursor::new(b"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526");
	
	let mut g1 = read_grid(cursor);
	let mut total_flashes = 0;
	for _ in 0..100 {
            let res = step(&mut g1);
	    total_flashes+=res;
	}
	assert_eq!(total_flashes,1656);
    }

}
