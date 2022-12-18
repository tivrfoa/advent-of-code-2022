use crate::util;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::ops::RangeInclusive;

fn calc_distance(point1: &Pos, point2: &Pos) -> i64 {
    (point1.col - point2.col).abs() + (point1.row - point2.row).abs()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    row: i64,
    col: i64,
}

impl Pos {
    fn new(col: i64, row: i64) -> Self {
        Pos { col, row }
    }
}

#[derive(Debug)]
struct Sensor {
    at: Pos,
    closest_beacon: Pos,
    distance_to_closest_beacon: i64,
    beacons_distances: HashMap<i64, Pos>,
}

impl Sensor {
    fn set_beacons_distances(&mut self, beacons_set: &HashSet<Pos>) {
        for b in beacons_set {
            let dist = calc_distance(&self.at, b);
            self.beacons_distances.insert(dist, *b);
        }
    }

    fn new(at: Pos, closest_beacon: Pos) -> Self {
        let distance_to_closest_beacon = calc_distance(&at, &closest_beacon);
        Sensor {
            at,
            closest_beacon,
            distance_to_closest_beacon,
            beacons_distances: HashMap::new(),
        }
    }

    fn radius(&self) -> i64 {
        self.distance_to_closest_beacon
    }

    fn get_min_x(&self) -> i64 {
        self.at.col.min(self.closest_beacon.col)
    }

    fn get_max_x(&self) -> i64 {
        self.at.col.max(self.closest_beacon.col)
    }

    fn get_min_y(&self) -> i64 {
        self.at.row.min(self.closest_beacon.row)
    }

    fn get_max_y(&self) -> i64 {
        self.at.row.max(self.closest_beacon.row)
    }
}

/// @return sensors and set of beacons
fn parse_input(input: String) -> (Vec<Sensor>, HashSet<Pos>) {
    let mut sensors = vec![];
    let mut beacons_set = HashSet::new();
    for line in input.lines() {
        let mut iter = line.split('=');
        iter.next();
        let x: i64 = iter
            .next()
            .unwrap()
            .split_once(',')
            .unwrap()
            .0
            .parse()
            .unwrap();
        let y: i64 = iter
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .0
            .parse()
            .unwrap();
        let bx: i64 = iter
            .next()
            .unwrap()
            .split_once(',')
            .unwrap()
            .0
            .parse()
            .unwrap();
        let by: i64 = iter.next().unwrap().parse().unwrap();
        sensors.push(Sensor::new(Pos::new(x, y), Pos::new(bx, by)));
        beacons_set.insert(Pos::new(bx, by));
    }

    (sensors, beacons_set)
}

fn get_min_max_x(sensors: &[Sensor]) -> (i64, i64) {
    let min_x = sensors.iter().map(|s| s.get_min_x()).min();
    let max_x = sensors.iter().map(|s| s.get_max_x()).max();

    (min_x.unwrap(), max_x.unwrap())
}

fn get_min_max_y(sensors: &[Sensor]) -> (i64, i64) {
    let min_y = sensors.iter().map(|s| s.get_min_y()).min();
    let max_y = sensors.iter().map(|s| s.get_max_y()).max();

    (min_y.unwrap(), max_y.unwrap())
}

fn find_first_beacon_in_row(sensors: &[Sensor], row_to_check: i64) -> i64 {
    for s in sensors {
        if s.closest_beacon.row == row_to_check {
            return s.closest_beacon.col;
        }
    }
    panic!("It didn't find beacon at row: {row_to_check}");
}

fn can_contain_beacon(sensors: &[Sensor], pos: Pos) -> bool {
    for s in sensors {
        let dist = calc_distance(&s.at, &pos);
        if dist <= s.distance_to_closest_beacon {
            return false;
        }
        // the other thing to consider is if dist is equal to that sensor
        // distance to an existing beacon, as there are no ties.
        // if s.beacons_distances.get(&dist).is_some() {
        // 	return false;
        // }
    }

    true
}

pub fn solve(input: String, row_to_check: i64) -> usize {
    let (mut sensors, beacons_set) = parse_input(input);
    let (min_col, max_col) = get_min_max_x(&sensors);

    //for s in &mut sensors {
    //    s.set_beacons_distances(&beacons_set);
    //}

    // which column to start ...? It helps that both rows to check
    // have beacon on it ... so I'll start from them
    let start_col = find_first_beacon_in_row(&sensors, row_to_check);

    let mut positions_without_beacon = 0;

    // count left
    let mut found_beacon = false;
    let mut curr_col = start_col - 1;
    while !found_beacon || curr_col >= min_col - 10 {
        if can_contain_beacon(&sensors, Pos::new(curr_col, row_to_check)) {
            found_beacon = true;
        } else {
            positions_without_beacon += 1;
        }
        curr_col -= 1;
    }
    println!("left: {}", positions_without_beacon);

    // count right
    let mut found_beacon = false;
    let mut curr_col = start_col + 1;
    while !found_beacon || curr_col <= max_col + 1 {
        if can_contain_beacon(&sensors, Pos::new(curr_col, row_to_check)) {
            found_beacon = true;
        } else {
            positions_without_beacon += 1;
        }
        curr_col += 1;
    }

    positions_without_beacon
}

struct SB {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

impl SB {
    fn get_sb_vec(sensors: &[Sensor]) -> Vec<Self> {
        let mut vec = Vec::with_capacity(sensors.len());
        for s in sensors {
            vec.push(SB {
                sx: s.at.col,
                sy: s.at.row,
                bx: s.closest_beacon.col,
                by: s.closest_beacon.row,
            });
        }
        vec
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Ev {
    x: i64,
    d: i64,
}

pub fn solve_part2(input: String, max: i64) -> i64 {
    let (sensors, beacons_set) = parse_input(input);
    let (min_col, max_col) = get_min_max_x(&sensors);
    let (min_row, max_row) = get_min_max_y(&sensors);

    // based on Elizarov solution
    // https://github.com/elizarov/AdventOfCode2022/blob/main/src/Day15.kt
    let sbs = SB::get_sb_vec(&sensors);
    let max_c = 4_000_000;
    let mut es: Vec<Ev> = vec![];
    let mut ans: i64 = 0;

    for ty in 0..max_c {
        es.clear();
        for SB { sx, sy, bx, by } in &sbs {
            let d = (sx - bx).abs() + (sy - by).abs();
            if (ty - sy).abs() <= d {
                let w = d - (ty - sy).abs();
                es.push(Ev { x: sx - w, d: 1 });
                es.push(Ev {
                    x: sx + w + 1,
                    d: -1,
                });
            }
        }

        es.sort();
        let mut px = es[0].x;
        let mut cnt = 0;
        for e in &es {
            if e.x > px {
                if cnt == 0 && px >= 0 && px <= max_c {
                    println!("{px} {ty}");
                    return px * 4_000_000 + ty;
                }
                px = e.x;
            }
            cnt += e.d;
        }
    }

    panic!("failed!");
}

#[allow(dead_code)]
fn dbg(grid: &Vec<Vec<char>>) {
    for item in grid {
        println!("{:?}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/day15-sample.txt");
        assert_eq!(26, solve(input, 10));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day15.txt");
        assert_eq!(5181556, solve(input, 2_000_000));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/day15-sample.txt");
        assert_eq!(56000011, solve_part2(input, 20));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/day15.txt");
        assert_eq!(12817603219131, solve_part2(input, 4000000));
    }
}
