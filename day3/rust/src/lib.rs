#[allow(unused_imports)]
use anyhow::{anyhow, Result};

use std::fs;
use std::cmp::*;

pub fn process_data(f_data: String) -> Result<(Vec<String>,Vec<String>)> {
	let mut lines = f_data.lines();

	let path1 = lines.next().unwrap()
		.split(',')
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.to_string())
		.collect();

	let path2 = lines.next().unwrap()
		.split(',')
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.to_string())
		.collect();

	Ok((path1, path2))
}

#[derive(Clone,Copy,Debug,PartialEq)]
struct Point {
	x: i64,
	y: i64,
}

impl Point {
	fn new(x: i64, y: i64) -> Point {
		Point {x: x, y: y}
	}

	fn dist(&self) -> u64 {
		self.x.abs() as u64 + self.y.abs() as u64
	}
}

#[allow(dead_code)]
#[derive(Debug)]
struct WirePath {
	a: Point,
	b: Point,
}

#[allow(dead_code)]
impl WirePath {
	fn new(a: Point, b: Point) -> WirePath {
		WirePath {a: a, b: b}
	}

	#[allow(dead_code)]
	fn from_raw(ax: i64, ay: i64, bx: i64, by: i64) -> WirePath {
		WirePath {a: Point::new(ax,ay), b: Point::new(bx, by)}
	}

	fn is_hor(&self) -> bool {
		self.a.y == self.b.y
	}

	fn is_vert(&self) -> bool {
		self.a.x == self.b.x
	}

	fn north(&self) -> i64 {
		max(self.a.y, self.b.y)
	}

	fn south(&self) -> i64 {
		min(self.a.y, self.b.y)
	}

	fn west(&self) -> i64 {
		min(self.a.x, self.b.x)
	}

	fn east(&self) -> i64 {
		max(self.a.x, self.b.x)
	}

	fn crosses(&self, wp: &WirePath) -> Option<Point> {
		if self.is_hor() == wp.is_hor() {
			return None;
		}
		let zero = Point::new(0,0);
		if zero == self.a || zero == wp.a {
			return None;
		}

		if self.is_hor() {
			if wp.north() >= self.north() && wp.south() <= self.south() &&
			   wp.east() <= self.east() && wp.west() >= self.west() {
			   	return Some(Point::new(wp.a.x, self.a.y));
			}
		} else {
			if self.north() >= wp.north() && self.south() <= wp.south() &&
			   self.east() <= wp.east() && self.west() >= wp.west()  {
			   	return Some(Point::new(self.a.x, wp.a.y));
			}
		}

		return None
	}
}

fn get_distance(paths1: &Vec<String>, paths2: &Vec<String>) -> Result<u64> {
    let mut p1 = Point::new(0,0);
    let mut p2: Point;
    let mut wp1: Vec<WirePath> = Vec::new();
    let mut dist = u64::MAX;

    for path in paths1 {
    	let dir = path.chars().next().expect("could not determine direction");
    	let val: i64 = path.get(1..).unwrap().parse()?;
    	p2 = match dir {
    		'R' => Point::new(p1.x + val, p1.y),
    		'L' => Point::new(p1.x - val, p1.y),
    		'U' => Point::new(p1.x, p1.y + val),
    		'D' => Point::new(p1.x, p1.y - val),
    		_ => panic!("which way?")
    	};

    	let wirepath = WirePath::new(p1, p2);
    	wp1.push(wirepath);
    	p1 = p2;
    }

	p1 = Point::new(0,0);
    for path in paths2 {
		let dir = path.chars().next().expect("could not determine direction");
    	let val: i64 = path.get(1..).unwrap().parse()?;
    	p2 = match dir {
    		'R' => Point::new(p1.x + val, p1.y),
    		'L' => Point::new(p1.x - val, p1.y),
    		'U' => Point::new(p1.x, p1.y + val),
    		'D' => Point::new(p1.x, p1.y - val),
    		_ => panic!("which way?")
    	};

    	let wirepath = WirePath::new(p1, p2);
    	for wp in &wp1 {
    		match wp.crosses(&wirepath) {
    			Some(p) => {
    				let pot_dist = p.dist();
    				if pot_dist < dist {
    					dist = pot_dist;
    				}
    			},
    			None => {},
    		}
    	}
    	p1 = p2
    }

    Ok(dist)
}

#[cfg(feature = "bench")]
pub fn gd(paths1: &Vec<String>, paths2: &Vec<String>) -> Result<u64> {
	get_distance(paths1, paths2)
}

pub fn day3(file: &str) -> Result<u64> {
	let f_data = fs::read_to_string(file).expect("didn't find the file");
	let (paths1, paths2) = process_data(f_data)?;
	Ok(get_distance(&paths1, &paths2)?)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_distance() {
		let mut s = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\n");
		s.push_str("U62,R66,U55,R34,D71,R55,D58,R83");
		let (paths1, paths2) = process_data(s).unwrap();
		assert_eq!(159, get_distance(&paths1, &paths2).unwrap());

		s = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n");
		s.push_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
		let (p1, p2) = process_data(s).unwrap();
		assert_eq!(135, get_distance(&p1, &p2).unwrap());	  
	}
}
