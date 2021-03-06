use anyhow::Result;

use std::cmp::*;
use std::fs;

pub fn process_data(f_data: String) -> Result<(Vec<String>, Vec<String>)> {
    let mut lines = f_data.lines();

    let path1 = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let path2 = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok((path1, path2))
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }

    fn manhattan_dist(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct WirePath {
    a: Point,
    b: Point,
    prev_latency: i64, // latency upto, but not including this wirepath
                       // ie latency of all previous wirepaths
}

#[allow(dead_code)]
impl WirePath {
    fn new(a: Point, b: Point) -> WirePath {
        WirePath {
            a: a,
            b: b,
            prev_latency: 0,
        }
    }

    #[allow(dead_code)]
    fn from_raw(ax: i64, ay: i64, bx: i64, by: i64) -> WirePath {
        WirePath {
            a: Point::new(ax, ay),
            b: Point::new(bx, by),
            prev_latency: 0,
        }
    }

    fn from_data(a: Point, path: &String) -> Result<(Point, WirePath)> {
        let dir = path.chars().next().expect("could not determine direction");
        let val: i64 = path.get(1..).unwrap().parse()?;
        let b = match dir {
            'R' => Point::new(a.x + val, a.y),
            'L' => Point::new(a.x - val, a.y),
            'U' => Point::new(a.x, a.y + val),
            'D' => Point::new(a.x, a.y - val),
            _ => panic!("which way?"),
        };

        let wirepath = WirePath::new(a, b);

        Ok((b, wirepath))
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
        let zero = Point::new(0, 0);
        if zero == self.a || zero == wp.a {
            return None;
        }

        if self.is_hor() {
            if wp.north() >= self.north()
                && wp.south() <= self.south()
                && wp.east() <= self.east()
                && wp.west() >= self.west()
            {
                return Some(Point::new(wp.a.x, self.a.y));
            }
        } else {
            if self.north() >= wp.north()
                && self.south() <= wp.south()
                && self.east() <= wp.east()
                && self.west() >= wp.west()
            {
                return Some(Point::new(self.a.x, wp.a.y));
            }
        }

        return None;
    }

    fn set_latency(&mut self, lat: i64) {
        self.prev_latency = lat
    }

    fn get_latency(&self) -> i64 {
        if self.is_hor() {
            return self.prev_latency + (self.west() - self.east()).abs();
        }
        return self.prev_latency + (self.north() - self.south()).abs();
    }

    fn get_prev_latency(&self) -> i64 {
        self.prev_latency
    }
}

fn no_op(_wps: Option<&Vec<WirePath>>, _pd: i64, _cwp: &WirePath) -> i64 {
    0
}

fn make_wirepath_set(
    paths: &Vec<String>,
    dist: impl Fn(Option<&Vec<WirePath>>, i64, &WirePath) -> i64,
    wp1: Option<&Vec<WirePath>>,
) -> Result<(Vec<WirePath>, i64)> {
    let mut p1 = Point::new(0, 0);
    let mut wps: Vec<WirePath> = Vec::new();
    let mut lat = 0;
    let mut dst = i64::MAX;

    for path in paths {
        let (p2, mut wirepath) = WirePath::from_data(p1, &path)?;
        wirepath.set_latency(lat);
        lat = wirepath.get_latency();
        dst = dist(wp1, dst, &wirepath);

        wps.push(wirepath);
        p1 = p2;
    }

    Ok((wps, dst))
}

fn manhattan_cross_dist(wps: Option<&Vec<WirePath>>, pd: i64, cwp: &WirePath) -> i64 {
    let mut dist: i64 = pd;

    let paths = match wps {
        Some(p) => p,
        _ => return dist
    };

    for wp in paths {
        let p = match wp.crosses(cwp) {
            Some(p) => p,
            _ => continue,
        };

        let pot_dist = p.manhattan_dist();
        if pot_dist < dist {
            dist = pot_dist;
        }
    }

    dist
}

fn get_manhattan(paths1: &Vec<String>, paths2: &Vec<String>) -> Result<i64> {
    let (wp1, _) = make_wirepath_set(&paths1, no_op, None)?;
    let (_, dist) = make_wirepath_set(&paths2, manhattan_cross_dist, Some(&wp1))?;

    Ok(dist)
}

#[cfg(feature = "bench")]
pub fn gm(paths1: &Vec<String>, paths2: &Vec<String>) -> Result<i64> {
    get_manhattan(paths1, paths2)
}

fn latency_cross_dist(wps: Option<&Vec<WirePath>>, pd: i64, cwp: &WirePath) -> i64 {
    let mut dist: i64 = pd;

    let paths = match wps {
        Some(wps) => wps,
        _ => return dist
    };

    for wp in paths {
        let p = match wp.crosses(cwp) {
            Some(p) => p,
            _ => continue,
        };

        let mut latency = wp.get_prev_latency() + cwp.get_prev_latency();
        if wp.is_hor() {
            latency += (wp.a.x - p.x).abs();
            latency += (cwp.a.y - p.y).abs();
        } else {
            latency += (wp.a.y - p.y).abs();
            latency += (cwp.a.x - p.x).abs();
        }

        if latency < dist {
            dist = latency;
        }
    }

    dist
}

fn get_latency(paths1: &Vec<String>, paths2: &Vec<String>) -> Result<i64> {
    let (wp1, _) = make_wirepath_set(&paths1, no_op, None)?;
    let (_, dist) = make_wirepath_set(&paths2, latency_cross_dist, Some(&wp1))?;

    Ok(dist)
}

#[cfg(feature = "bench")]
pub fn gl(paths1: &Vec<String>, paths2: &Vec<String>) -> Result<i64> {
    get_latency(paths1, paths2)
}

pub fn day3(file: &str) -> Result<(i64, i64)> {
    let f_data = fs::read_to_string(file).expect("didn't find the file");
    let (paths1, paths2) = process_data(f_data)?;
    let manhattan = get_manhattan(&paths1, &paths2)?;
    let latency = get_latency(&paths1, &paths2)?;
    Ok((manhattan, latency))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manhattan() {
        let mut s = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\n");
        s.push_str("U62,R66,U55,R34,D71,R55,D58,R83");
        let (paths1, paths2) = process_data(s).unwrap();
        assert_eq!(159, get_manhattan(&paths1, &paths2).unwrap());

        s = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n");
        s.push_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let (p1, p2) = process_data(s).unwrap();
        assert_eq!(135, get_manhattan(&p1, &p2).unwrap());
    }

    #[test]
    fn test_latency() {
        let mut s = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\n");
        s.push_str("U62,R66,U55,R34,D71,R55,D58,R83");
        let (paths1, paths2) = process_data(s).unwrap();
        let (wps, _) = make_wirepath_set(&paths1, no_op, None).unwrap();

        assert_eq!(0, wps[0].get_prev_latency());
        assert_eq!(75, wps[0].get_latency());
        assert_eq!(75, wps[1].get_prev_latency());
        assert_eq!(105, wps[1].get_latency());
        assert_eq!(105, wps[2].get_prev_latency());
        assert_eq!(188, wps[2].get_latency());
        assert_eq!(188, wps[3].get_prev_latency());
        assert_eq!(271, wps[3].get_latency());
        assert_eq!(271, wps[4].get_prev_latency());
        assert_eq!(283, wps[4].get_latency());
        assert_eq!(283, wps[5].get_prev_latency());
        assert_eq!(332, wps[5].get_latency());
        assert_eq!(332, wps[6].get_prev_latency());
        assert_eq!(403, wps[6].get_latency());
        assert_eq!(403, wps[7].get_prev_latency());
        assert_eq!(410, wps[7].get_latency());
        assert_eq!(410, wps[8].get_prev_latency());
        assert_eq!(482, wps[8].get_latency());

        let (wps, _) = make_wirepath_set(&paths2, no_op, None).unwrap();

        assert_eq!(62, wps[0].get_latency());
        assert_eq!(128, wps[1].get_latency());
        assert_eq!(183, wps[2].get_latency());
        assert_eq!(217, wps[3].get_latency());
        assert_eq!(288, wps[4].get_latency());
        assert_eq!(343, wps[5].get_latency());
        assert_eq!(401, wps[6].get_latency());
        assert_eq!(484, wps[7].get_latency());
    }

    #[test]
    fn test_get_latency() {
        let mut s = String::from("R8,U5,L5,D3\n");
        s.push_str("U7,R6,D4,L4");

        let (p1, p2) = process_data(s).unwrap();
        assert_eq!(30, get_latency(&p1, &p2).unwrap());

        let mut s = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72\n");
        s.push_str("U62,R66,U55,R34,D71,R55,D58,R83");
        let (paths1, paths2) = process_data(s).unwrap();
        assert_eq!(610, get_latency(&paths1, &paths2).unwrap());

        let mut s = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n");
        s.push_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        let (paths1, paths2) = process_data(s).unwrap();
        assert_eq!(410, get_latency(&paths1, &paths2).unwrap());
    }
}
