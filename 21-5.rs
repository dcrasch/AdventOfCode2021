use std::collections::HashMap;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x_fromstr = coords[0].parse::<i64>()?;
        let y_fromstr = coords[1].parse::<i64>()?;
        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    p1: Point,
    p2: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<&str> = s.split(" -> ").collect();
        let p1_fromstr = points[0].parse::<Point>()?;
        let p2_fromstr = points[1].parse::<Point>()?;
        Ok(Line {
            p1: p1_fromstr,
            p2: p2_fromstr,
        })
    }
}

impl Line {
    fn points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let sx = self.p1.x.min(self.p2.x);
            let ex = self.p1.x.max(self.p2.x);
            let y = self.p1.y;
            return (sx..ex + 1).map(|x| Point { x, y }).collect();
        } else if self.is_vertical() {
            let x = self.p1.x;
            let sy = self.p1.y.min(self.p2.y);
            let ey = self.p1.y.max(self.p2.y);
            return (sy..ey + 1).map(|y| Point { x, y }).collect();
        } else {
            let dx = self.p2.x - self.p1.x;
            let dy = self.p2.y - self.p1.y;
            let vx = if dx > 0 { 1 } else { -1 };
            let vy = if dy > 0 { 1 } else { -1 };
            let mut points: Vec<Point> = Vec::new();
            let mut x = self.p1.x;
            let mut y = self.p1.y;
            for _ in 0..dx.abs() + 1 {
                points.push(Point { x: x, y: y });
                x += vx;
                y += vy;
            }
            points
        }
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }
    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
}
fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut stdlines = reader.lines();

    let lines: Vec<Line> = stdlines
        .map(|x| x.unwrap().parse().expect("parse error"))
        .collect();

    let mut overlaps: HashMap<Point, usize> = HashMap::new();
    for line in lines {
        let points = line.points();
        for point in points {
            *overlaps.entry(point).or_insert(0) += 1;
        }
    }
    let two_overlaps_count = overlaps.values().filter(|&&c| c >= 2).count();
    println!("sol1 :{}", two_overlaps_count);
}
