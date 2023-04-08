use crate::util;

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::zip;

#[derive(Debug)]
struct Cave<'a> {
    connections: Vec<&'a str>,
    small: bool,
}

impl<'a> Cave<'a> {
    fn new(label: &'a str) -> Self {
        Self {
            connections: vec![],
            small: label.chars().next().unwrap().is_lowercase(),
        }
    }

    fn add_conn(&mut self, conn: &'a str) {
        self.connections.push(conn);
    }
}

fn visit<'a>(caves: &HashMap<&'a str, Cave<'a>>, key: &str, prev: &str,
        visited: &mut HashSet<&'a str>) -> u32 {
    let cave = caves.get(key).unwrap();

    let mut paths = 0;
    for conn in &cave.connections {
        let is_conn_small = is_lowercase(conn);
        let is_key_small = is_lowercase(key);
        if visited.contains(conn) || (*conn == prev && !is_key_small) {
            continue;
        }

        if *conn == "end" {
            paths += 1;
            continue;
        }

        if is_conn_small {
            visited.insert(conn);
        }
        paths += visit(caves, conn, key, visited);
        visited.remove(conn);
    }

    paths
}

fn visit2<'a>(caves: &HashMap<&'a str, Cave<'a>>, key: &str, prev: &str,
        visited: &mut HashMap<&'a str, u8>, has_two: bool, curr_path: &String) -> u32 {
    let cave = caves.get(key).unwrap();
    let mut new_path = curr_path.clone();
    new_path.push_str(key);
    println!("New path: {}", new_path);

    if new_path == "startbd" {
        println!("..");
    }

    let mut qt_paths = 0;
    for conn in &cave.connections {
        if *conn == "start" {
            continue;
        }
        if *conn == "end" {
            qt_paths += 1;
            continue;
        }

        let is_conn_small = is_lowercase(conn);
        let is_key_small = is_lowercase(key);
        let mut next_two = has_two;

        // avoid a big cave going back to the same big cave
        if *conn == prev && !is_key_small && !is_conn_small {
            continue;
        }

        if is_conn_small {
            match visited.get_mut(conn) {
                Some(qt) => {
                    if *qt > 0 && next_two {
                        continue;
                    }
                    *qt += 1;
                    if *qt == 2 {
                        next_two = true;
                    }
                }
                None => {
                    visited.insert(conn, 1);
                }
            }
        }

        qt_paths += visit2(caves, conn, key, visited, next_two, &new_path);
        if let Some(qt) = visited.get_mut(conn) {
            *qt -= 1;
        }
    }

    qt_paths
}

fn is_lowercase(s: &str) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

fn part1(input: String) -> String {
    let mut caves: HashMap<&str, Cave> = HashMap::new();

    for line in input.lines() {
        let tmp = line.split_once('-').unwrap();
        let left = caves.entry(tmp.0).or_insert_with(|| Cave::new(tmp.0));
        left.add_conn(tmp.1);
        let right = caves.entry(tmp.1).or_insert_with(|| Cave::new(tmp.1));
        right.add_conn(tmp.0);
    }
    // dbg!(caves);

    // it needs to track the previous cave, to avoid a big cave going
    // back to the same big cave

    // keep track of visited small caves, and don't visit them again

    // if it gets to the end, increment the answer

    let mut visited = HashSet::new();
    visited.insert("start");

    visit(&caves, "start", "start", &mut visited).to_string()
}

fn part2(input: String) -> String {
    let mut caves: HashMap<&str, Cave> = HashMap::new();

    for line in input.lines() {
        let tmp = line.split_once('-').unwrap();
        let left = caves.entry(tmp.0).or_insert_with(|| Cave::new(tmp.0));
        left.add_conn(tmp.1);
        let right = caves.entry(tmp.1).or_insert_with(|| Cave::new(tmp.1));
        right.add_conn(tmp.0);
    }

    visit2(&caves, "start", "start", &mut HashMap::new(), false, &String::new()).to_string()
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
fn get_dirs_with_diagonals(r: usize, c: usize, rows: usize, cols: usize) -> [(bool, (usize, usize)); 8] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1s() {
        let input = util::read_file("inputs/2021/day12-sample.txt");
        assert_eq!("10", part1(input));
    }

    #[test]
    fn p1() {
        let input = util::read_file("inputs/2021/day12.txt");
        assert_eq!("4885", part1(input));
    }

    #[test]
    fn p2s() {
        let input = util::read_file("inputs/2021/day12-sample.txt");
        assert_eq!("36", part2(input));
    }

    #[test]
    fn p2() {
        let input = util::read_file("inputs/2021/day12.txt");
        assert_eq!("117095", part2(input));
    }
}
