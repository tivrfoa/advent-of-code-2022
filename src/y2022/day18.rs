use crate::util;

use std::collections::HashSet;

use std::cmp::Ordering;

/*


  - A cube has 6 sides
  - How to check if they are connected?
    It seems you can consider they connected if they have two equal axisses.
  - Can a cube have all its sides covered?



  - Input has 2191 cubes
    . I can sort it twice:
     1st by x, y, z, then it can compare xy and yz
     2nd by x, z, y, then it compares xz

  Approach 1:
    For each cube, compute how many sides are covered;
    Then in the end, loop through all cubes summing: 6 - sides_covered

*/

//#[derive(Debug)]
//struct Cube {
//    x: usize,
//    y: usize,
//    z: usize,
//}
//
//impl Cube {
//    fn new(x: usize, y: usize, z: usize) -> Self {
//        Self {
//            x,
//            y,
//            z,
//        }
//    }
//}

pub fn solve(input: String) -> usize {
    let mut cubes: HashSet<(i32, i32, i32)> = parse(input);

    // copied solution from UncleScientist
    // I had no idea how to do that

    let mut sides = cubes.len() * 6;
    let delta_xyz = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];

    for c in &cubes {
        for d in delta_xyz {
            let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
            if cubes.contains(&pos) {
                sides -= 1;
            }
        }
    }

    sides
}

pub fn solve_part2(input: String) -> usize {
    0
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
        let input = util::read_file("inputs/2022/day18-sample.txt");
        assert_eq!(64, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day18.txt");
        assert_eq!(3374, solve(input));
    }

    //#[test]
    //fn part2_sample() {
    //    // Approaches for part 2:
    //    //   1. maybe 1 lines is enough, then I can have a 2002 lines grid
    //    //      and when it crosses line 2000, I copy the last 1000 lines to
    //    //      the begenning the clear the rest
    //    //   2. Detect when the patter repeats and then just multiply for the
    //    //      remaining stones
    //    let input = util::read_file("inputs/2022/day18-sample.txt");
    //    assert_eq!(1514285714288, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/2022/day18.txt");
    //    assert_eq!(1537175792495, solve_part2(input));
    //}
}

fn parse(input: String) -> HashSet<(i32, i32, i32)> {
    let mut cubes = HashSet::new();
    for line in input.lines() {
        let tokens: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
        let x = tokens[0];
        let y = tokens[1];
        let z = tokens[2];
        cubes.insert((x, y, z));
    }
    cubes
}
