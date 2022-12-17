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
fn calc_distance(point1: Pos, point2: Pos) -> i32 {
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

pub fn solve(input: String, row_to_check: i32) -> usize {
	let sensors = get_sensors(input);
	let (min_x, max_x) = get_min_max_x(&sensors);
	dbg!(&sensors);
	dbg!(min_x, max_x);

	0
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
