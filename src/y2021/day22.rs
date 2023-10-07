use crate::util;

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::zip;

type Range = (i32, i32);

#[derive(Debug)]
struct Step {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Step {
    fn new(on: bool, x: Range, y: Range, z: Range) -> Self {
        Self { on, x, y, z }
    }
}

fn get_range(s: &str) -> (i32, i32) {
    let (_, s) = s.split_once('=').unwrap();
    let (a, b) = s.split_once("..").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn part1(input: String) -> String {
    let mut steps: Vec<Step> = vec![];
    let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();
    for line in input.lines().take(20) {
        let xyz = line.split(",").collect::<Vec<&str>>();
        let (on, _) = xyz[0].split_once(' ').unwrap();
        steps.push(Step::new(
            on == "on",
            get_range(&xyz[0]),
            get_range(&xyz[1]),
            get_range(&xyz[2]),
        ));
    }
    // dbg!(steps);
    for step in steps {
        for x in step.x.0..=step.x.1 {
            for y in step.y.0..=step.y.1 {
                for z in step.z.0..=step.z.1 {
                    if step.on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes.len().to_string()
}

fn map(vec: &[i32]) -> HashMap<i32, usize> {
	let mut hm: HashMap<i32, usize> = HashMap::new();
	for (i, v) in vec.iter().enumerate() {
		hm.insert(*v, i);
	}
	hm
}
fn part2(input: String) -> String {
    let mut steps: Vec<Step> = vec![];
    for line in input.lines() {
        let xyz = line.split(",").collect::<Vec<&str>>();
        let (on, _) = xyz[0].split_once(' ').unwrap();
        let (x1, x2) = get_range(&xyz[0]);
        let (y1, y2) = get_range(&xyz[1]);
        let (z1, z2) = get_range(&xyz[2]);
        steps.push(Step::new(
            on == "on",
            (x1, x2 + 1),
            (y1, y2 + 1),
            (z1, z2 + 1),
        ));
    }

	let ux = sort_uniq(&steps, |s| vec![s.x.0, s.x.1]);
	let uy = sort_uniq(&steps, |s| vec![s.y.0, s.y.1]);
	let uz = sort_uniq(&steps, |s| vec![s.z.0, s.z.1]);
	let mut g = vec![vec![vec![false; uz.len()]; uy.len()]; ux.len()];
	let mx = map(&ux);
	let my = map(&uy);
	let mz = map(&uz);
	for s in steps {
		for x in mx[&s.x.0]..mx[&s.x.1] {
			for y in my[&s.y.0]..my[&s.y.1] {
				for z in mz[&s.z.0]..mz[&s.z.1] {
					g[x][y][z] = s.on;
				}
			}
		}
	}
	let mut ans = 0;
	for x in 0..ux.len() {
		for y in 0..uy.len() {
			for z in 0..uz.len() {
				if g[x][y][z] {
					let x = (ux[x + 1] - ux[x]) as i64;
					let y = (uy[y + 1] - uy[y]) as i64;
					let z = (uz[z + 1] - uz[z]) as i64;
					ans += x * y * z;
				}
			}
		}
	}

	ans.to_string()
}

fn sort_uniq(steps: &[Step], f: fn(&Step) -> Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = steps
		.into_iter()
		.flat_map(|s| f(s))
		.collect();
	v.sort();
	v.dedup();
	v
}

// https://www.reddit.com/r/adventofcode/comments/rlxhmg/2021_day_22_solutions/hpiz583/
fn part2_2(input: String) -> String {
    let mut steps: Vec<Step> = vec![];
    for line in input.lines() {
        let xyz = line.split(",").collect::<Vec<&str>>();
        let (on, _) = xyz[0].split_once(' ').unwrap();
        let (x1, x2) = get_range(&xyz[0]);
        let (y1, y2) = get_range(&xyz[1]);
        let (z1, z2) = get_range(&xyz[2]);
        steps.push(Step::new(
            on == "on",
            (x1, x2),
            (y1, y2),
            (z1, z2),
        ));
    }
	let mut cubes: Vec<(i32, i32, i32, i32, i32, i32)> = vec![];

	for s in steps {
		let op = s.on;
		let (ux, vx) = s.x;
		let (uy, vy) = s.y;
		let (uz, vz) = s.z;

		let mut new_cubes = vec![];
		for i in 0..cubes.len() {
			let (ux2, vx2, uy2, vy2, uz2, vz2) = cubes[i];
			if ux > vx2 || vx < ux2 || uy > vy2 || vy < uy2 || uz > vz2 || vz < uz2 {
				new_cubes.push((ux2, vx2, uy2, vy2, uz2, vz2));
				continue;
			}
			if ux > ux2 {
				new_cubes.push((ux2, ux - 1, uy2, vy2, uz2, vz2));
			}
			if vx < vx2 {
				new_cubes.push((vx + 1, vx2, uy2, vy2, uz2, vz2));
			}
			if uy > uy2 {
				new_cubes.push((max(ux2, ux), min(vx2, vx), uy2, uy - 1, uz2, vz2));
			}
			if vy < vy2 {
				new_cubes.push((max(ux2, ux), min(vx2, vx), vy + 1, vy2, uz2, vz2));
			}
			if uz > uz2 {
				new_cubes.push((max(ux2, ux), min(vx2, vx), max(uy2, uy), min(vy2, vy), uz2, uz - 1));
			}
			if vz < vz2 {
				new_cubes.push((max(ux2, ux), min(vx2, vx), max(uy2, uy), min(vy2, vy), vz + 1, vz2));
			}
		}
		if op {
			// new_cubes.push((min(ux, vx), max(ux, vx), min(uy, vy), max(uy, vy), min(uz, vz), max(uz, vz)));
			new_cubes.push((ux, vx, uy, vy, uz, vz));
		}
		cubes = new_cubes;
	}

	let mut on_count: i64 = 0;
	for (ux, vx, uy, vy, uz, vz) in cubes {
		on_count += (vx - ux + 1) as i64 * (vy - uy + 1) as i64 * (vz - uz + 1) as i64;
	}

	on_count.to_string()
}

fn max(a: i32, b: i32) -> i32 {
	a.max(b)
}

fn min(a: i32, b: i32) -> i32 {
	a.min(b)
}

#[allow(dead_code)]
fn dbg_grid<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}

#[allow(dead_code)]
fn in_to_nums<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input.split(',').map(|n| n.parse::<T>().unwrap()).collect()
}

#[allow(dead_code)]
fn split_str_to_nums<T: std::str::FromStr>(input: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: Debug,
{
    input
        .split(separator)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn vec_max<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().max().unwrap()
}

#[allow(dead_code)]
fn vec_min<T: std::str::FromStr + std::cmp::Ord + Copy>(vec: &[T]) -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    *vec.iter().min().unwrap()
}

#[allow(dead_code)]
fn str_to_char_tuple(s: &str) -> (char, char) {
    (
        s[0..1].chars().next().unwrap(),
        s[1..2].chars().next().unwrap(),
    )
}

#[allow(dead_code)]
trait MapAddOrInsert<K, V> {
    fn add_or_insert(&mut self, k: K, v: V);
}

#[allow(dead_code)]
impl<K: Eq + Hash, V: std::ops::AddAssign + Copy> MapAddOrInsert<K, V> for HashMap<K, V> {
    fn add_or_insert(&mut self, k: K, v: V) {
        self.entry(k).and_modify(|qt| *qt += v).or_insert(v);
    }
}

#[allow(dead_code)]
fn get_dirs(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 4] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
    ]
}

#[allow(dead_code)]
fn get_dirs_with_diagonals(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> [(bool, (usize, usize)); 8] {
    [
        // left
        (c > 0, (r, if c > 0 { c - 1 } else { 0 })),
        // right
        (c < cols - 1, (r, c + 1)),
        // top
        (r > 0, (if r > 0 { r - 1 } else { 0 }, c)),
        // bottom
        (r < rows - 1, (r + 1, c)),
        // top left
        (
            r > 0 && c > 0,
            (if r > 0 { r - 1 } else { 0 }, if c > 0 { c - 1 } else { 0 }),
        ),
        // top right
        (
            r > 0 && c < cols - 1,
            (if r > 0 { r - 1 } else { 0 }, c + 1),
        ),
        // bottom left
        (
            r < rows - 1 && c > 0,
            (r + 1, if c > 0 { c - 1 } else { 0 }),
        ),
        // bottom right
        (r < rows - 1 && c < cols - 1, (r + 1, c + 1)),
    ]
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day22-sample.txt");
        assert_eq!("590784", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day22.txt");
        assert_eq!("587097", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day22-sample2.txt");
        assert_eq!("2758514936282235", part2(input));
    }

    #[test]
#[ignore]
    fn p2() {
        let input = util::read_file("inputs/2021/day22.txt");
        assert_eq!("1359673068597669", part2(input));
    }

    #[test]
    fn p2_2() {
        let input = util::read_file("inputs/2021/day22.txt");
        assert_eq!("1359673068597669", part2_2(input));
    }
}
