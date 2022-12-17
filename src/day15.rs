use crate::util;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;
use std::ops::RangeInclusive;

/*

There are negative numbers, so it's best to avoid usize

Columns is actually unbounded.
So it's not infite because at some point (column) there might be
only beacon afterwards, then you just stop.

So how to calculate distance between two points?

The taxicab distance between two points ( x 1 , y 1 ) and ( x 2 , y 2 )
is | x 1 − x 2 | + | y 1 − y 2 | . That is, it is the sum of the absolute
values of the differences in both coordinates.

If a distance between an unvisited point and a sensor is lower
than the sensor closest beacon, then it cannot be a beacon.
So it needs to do this for all sensors, and if it is greater than all
sensors' beacon, the it might be a beacon.

So the remaining question is at which point to stop looking for positions
that do not contain a beacon?
Maybe it can stop when it might be a beacon and after it's x position is:
  - greater than all beacons when looking right;
  - smaller than all beacons when looking left.

*/
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

pub fn solve_part2(input: String, max: i64) -> i64 {
    let (sensors, beacons_set) = parse_input(input);
    let (min_col, max_col) = get_min_max_x(&sensors);
    let (min_row, max_row) = get_min_max_y(&sensors);

    // let's try to use ranges. Copying Login from Uncle Scientist
    // https://www.youtube.com/watch?v=oOgDTx_tAp4
    let mut rowdata: Vec<Vec<RangeInclusive<i64>>> = vec![vec![0..=max]; max as usize + 1];
    for s in &sensors {
        let radius = s.radius();
        let top = 0.max(s.at.row - radius);
        let bottom = max.min(s.at.row + radius);

        for row in top..=bottom {
            let dist = (s.at.row - row).abs();
            let min_x = 0.max(s.at.col - (radius - dist));
            let max_x = max.min(s.at.col + (radius - dist));

            let mut new_range = vec![];
            for r in &rowdata[row as usize] {
                let start = *r.start();
                if start > max_x {
                    new_range.push(r.clone());
                    continue;
                }

                let end = *r.end();
                if end < min_x {
                    new_range.push(r.clone());
                    continue;
                }

                if start < min_x {
                    new_range.push(start..=min_x - 1);
                }
                if end > max_x {
                    new_range.push(max_x + 1..=end);
                }
            }

            rowdata[row as usize] = new_range;
        }
    }

    for (y, r) in rowdata.iter().enumerate() {
        if !r.is_empty() {
            let x = r[0].start();
            return x * 4_000_000 + y as i64;
        }
    }

    panic!("It didn't find a beacon :(");
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
