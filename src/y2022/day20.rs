use crate::util;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};

/*

We have to decrypt the file.

The numbers should be moved in the order they originally
appear in the encrypted file.

the grove coordinates can be found by looking at the 1000th, 2000th, and
3000th numbers after the value 0, wrapping around the list as necessary.


1) when rotate right (final position > length), then just get the position and
shift right by 1 the elements from that position to prev_pos, eg:
  - moving 4
  - 1, 2, -3, 0, 3, 4, -2 -> 1, 2, -3, 4, 0, 3, -2
  - 4 idx is 6
  - 6 + 4 = 10
  - length = 7
  - 10 - 7 = 3 -> final position
  - shift right 1 final_pos..prev_pos
  - store value in final_pos

2) When rotate left (curr_pos - move < 0), eg:
  - moving -3
  - 1, -3, 2, 3, -2, 0, 4 -> 1, 2, 3, -2, -3, 0, 4
  - curr_pos: 1
  - 1 - 3 = -2
  - -2 < 0, then final_pos = len - 2 = 5
  - shift left 1 prev_pos+1..=final_pos
  - store value in final_pos

3) when going left and final position is 0, it must go to the last position in
the array, eg:
  - moving -2
  - 1, 2, -2, -3, 0, 3, 4 -> 1, 2, -3, 0, 3, 4, -2
  - shift left 1 curr_pos + 1..len
  - store value in last position


4) If none of the previous conditions, then just swap and update new positions

*/

fn trim_value(n: i32, len: usize) -> i32 {
    n % (len as i32 - 1)
}

fn mix_numbers(numbers: &mut [(i32, usize)], curr_positions: &mut [usize]) {
    let len = numbers.len();

    for i in 0..len {
        let curr_pos = curr_positions[i];
        let (value, original_index) = numbers[curr_pos];

        if value == 0 {
            continue;
        }

        if value > 0 {
            let final_pos = curr_pos + value as usize;

            if final_pos >= len {
                // rotate right
                let final_pos = value as usize - (len - curr_pos) + 1;
                // assert!(final_pos < curr_pos);
                for j in (final_pos..curr_pos).rev() {
                    numbers[j + 1] = numbers[j];
                    curr_positions[numbers[j].1] = j + 1;
                }
                numbers[final_pos] = (value, original_index);
                curr_positions[i] = final_pos;
            } else {
                for j in curr_pos..final_pos {
                    numbers[j] = numbers[j + 1];
                    curr_positions[numbers[j + 1].1] = j;
                }
                numbers[final_pos] = (value, original_index);
                curr_positions[i] = final_pos;
            }
        } else {
            let final_pos = curr_pos as i32 + value;
            if final_pos < 0 {
                let final_pos = (len as i32 + final_pos - 1) as usize;
                // println!("curr_pos: {curr_pos}, value: {value}");
                for j in curr_pos + 1..=final_pos {
                    numbers[j - 1] = numbers[j];
                    curr_positions[numbers[j].1] = j - 1;
                }
                numbers[final_pos] = (value, original_index);
                curr_positions[i] = final_pos;
            } else if final_pos == 0 {
                for j in curr_pos + 1..len {
                    numbers[j - 1] = numbers[j];
                    curr_positions[numbers[j].1] = j - 1;
                }
                numbers[len - 1] = (value, original_index);
                curr_positions[i] = len - 1;
            } else {
                let final_pos = curr_pos - value.abs() as usize;
                for j in (final_pos..curr_pos).rev() {
                    numbers[j + 1] = numbers[j];
                    curr_positions[numbers[j].1] = j + 1;
                }
                numbers[final_pos] = (value, original_index);
                curr_positions[i] = final_pos;
            }
        }
    }
}

pub fn part1(input: String) -> String {
    let numbers = parse(input);
    let original = numbers.clone();
    let len = numbers.len();
    let mut numbers: Vec<(i32, usize)> = numbers.into_iter().zip(0..len).collect();
    let mut curr_positions: Vec<usize> = (0..len).collect();

    numbers = numbers
        .into_iter()
        .map(|t| (trim_value(t.0, len), t.1))
        .collect();

    mix_numbers(&mut numbers, &mut curr_positions);

    //dbg!(&numbers);

    // find 0 value position
    let zero_index = numbers.iter().position(|t| t.0 == 0).unwrap();
    let sum = original[numbers[(zero_index + 1000) % len].1]
        + original[numbers[(zero_index + 2000) % len].1]
        + original[numbers[(zero_index + 3000) % len].1];

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let numbers = parse(input);
    let original = numbers.clone();
    let len = numbers.len();
    let mut numbers: Vec<(i32, usize)> = numbers.into_iter().zip(0..len).collect();
    let mut curr_positions: Vec<usize> = (0..len).collect();

    numbers = numbers
        .into_iter()
        .map(|t| ((t.0 as i64 * 811589153 % (len as i64 - 1)) as i32, t.1))
        .collect();

    for _ in 0..10 {
        mix_numbers(&mut numbers, &mut curr_positions);
    }

    //dbg!(&numbers);

    // find 0 value position
    let zero_index = numbers.iter().position(|t| t.0 == 0).unwrap();
    let sum = original[numbers[(zero_index + 1000) % len].1] as i64 * 811589153
        + original[numbers[(zero_index + 2000) % len].1] as i64 * 811589153
        + original[numbers[(zero_index + 3000) % len].1] as i64 * 811589153;

    sum.to_string()
}

fn parse(input: String) -> Vec<i32> {
    let mut numbers = Vec::with_capacity(5000);

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2022/day20-sample.txt");
        assert_eq!("3", part1(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day20.txt");
        assert_eq!("19559", part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2022/day20-sample.txt");
        assert_eq!("1623178306", part2(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2022/day20.txt");
        assert_eq!("912226207972", part2(input));
    }
}

#[allow(dead_code)]
fn dbg<T: Debug + Display>(grid: &[Vec<T>]) {
    for item in grid {
        println!("{item:?}");
    }
}
