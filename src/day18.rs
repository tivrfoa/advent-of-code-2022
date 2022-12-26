use crate::util;

use std::collections::HashMap;


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

pub fn solve(input: String) -> usize {




    todo!()
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
        let input = util::read_file("inputs/day18-sample.txt");
        assert_eq!(3068, solve(input));
    }

    //#[test]
    //fn part1_input() {
    //    let input = util::read_file("inputs/day18.txt");
    //    assert_eq!(3106, solve(input));
    //}

    //#[test]
    //fn part2_sample() {
    //    // Approaches for part 2:
    //    //   1. maybe 1 lines is enough, then I can have a 2002 lines grid
    //    //      and when it crosses line 2000, I copy the last 1000 lines to
    //    //      the begenning the clear the rest
    //    //   2. Detect when the patter repeats and then just multiply for the
    //    //      remaining stones
    //    let input = util::read_file("inputs/day18-sample.txt");
    //    assert_eq!(1514285714288, solve_part2(input));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day18.txt");
    //    assert_eq!(1537175792495, solve_part2(input));
    //}
}
