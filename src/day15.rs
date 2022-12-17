use crate::util;

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
fn calc_distance(point1: &Pos, point2: &Pos) -> i32 {
	(point1.col - point2.col).abs() + (point1.row - point2.row).abs()
}

#[derive(Debug)]
struct Pos {
	row: i32,
	col: i32,
}

impl Pos {
	fn new(col: i32, row: i32) -> Self {
		Pos {
			col,
			row,
		}
	}
}

#[derive(Debug)]
struct Sensor {
	at: Pos,
	closest_beacon: Pos,
}

impl Sensor {
	fn new(at: Pos, closest_beacon: Pos) -> Self {
		Sensor {
			at,
			closest_beacon,
		}
	}

	fn get_min_x(&self) -> i32 {
		self.at.col.min(self.closest_beacon.col)
	}

	fn get_max_x(&self) -> i32 {
		self.at.col.max(self.closest_beacon.col)
	}
}

fn get_sensors(input: String) -> Vec<Sensor> {
	let mut sensors = vec![];
    for line in input.lines() {
		let mut tmp1 = line.split("=");
		tmp1.next();
		let x: i32 = tmp1.next().unwrap().split_once(',').unwrap().0.parse().unwrap();
		let y: i32 = tmp1.next().unwrap().split_once(':').unwrap().0.parse().unwrap();
		let bx: i32 = tmp1.next().unwrap().split_once(',').unwrap().0.parse().unwrap();
		let by: i32 = tmp1.next().unwrap().parse().unwrap();
		sensors.push(Sensor::new(Pos::new(x, y), Pos::new(bx, by)));
    }

	sensors
}

fn get_min_max_x(sensors: &[Sensor]) -> (i32, i32) {
	let min_x = sensors.iter().map(|s| s.get_min_x()).min();
	let max_x = sensors.iter().map(|s| s.get_max_x()).max();

	(min_x.unwrap(), max_x.unwrap())
}

fn find_first_beacon_in_row(sensors: &[Sensor], row_to_check: i32) -> i32 {
	for s in sensors {
		if s.closest_beacon.row == row_to_check {
			return s.closest_beacon.col;
		}
	}
	panic!("It didn't find beacon at row: {row_to_check}");
}

fn can_contain_beacon(sensors: &[Sensor], pos: Pos) -> bool {

	for s in sensors {
		let dist1 = calc_distance(&s.at, &s.closest_beacon);
		let dist2 = calc_distance(&s.at, &pos);
		if dist2 < dist1 {
			return false;
		}
	}

	true
}

pub fn solve(input: String, row_to_check: i32) -> usize {
	let sensors = get_sensors(input);
	let (min_col, max_col) = get_min_max_x(&sensors);

	// which column to start ...? It helps that both rows to check
	// have beacon on it ... so I'll start from them
	let start_col = find_first_beacon_in_row(&sensors, row_to_check);

	let mut positions_without_beacon = 0;

	// count left
	let mut found_beacon = false;
	let mut curr_col = start_col - 1;
	while !found_beacon || curr_col >= min_col {
		if can_contain_beacon(&sensors, Pos::new(curr_col, row_to_check)) {
			found_beacon = true;
		} else {
			positions_without_beacon += 1;
		}
		curr_col -= 1;
	}

	// count right
	let mut found_beacon = false;
	let mut curr_col = start_col + 1;
	while !found_beacon || curr_col <= max_col {
		if can_contain_beacon(&sensors, Pos::new(curr_col, row_to_check)) {
			found_beacon = true;
		} else {
			positions_without_beacon += 1;
		}
		curr_col += 1;
	}


	positions_without_beacon
}

pub fn solve_part2(input: String) -> usize {
	todo!()
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
        assert_eq!(24, solve(input, 10));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day15.txt");
    //    assert_eq!(683, solve(input, 2_000_000));
    //}

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day15-sample.txt");
    //    assert_eq!(93, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day15.txt");
    //    assert_eq!(28821, solve_part2(input));
    //}
}
