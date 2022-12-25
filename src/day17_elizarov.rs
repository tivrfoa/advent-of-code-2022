use crate::util;

use std::collections::HashMap;

/*fn shape<T>(a: usize, u: T) -> bool
where T: Fn(usize, usize) -> bool {
    match a {
        0 => u(0, 0) && u(0, 1) && u(0, 2) && u(0, 3),
        1 => u(0, 1) && u(1, 0) && u(1, 1) && u(1, 2) && u(2, 1),
        2 => u(0, 0) && u(0, 1) && u(0, 2) && u(1, 2) && u(2, 2),
        3 => u(0, 0) && u(1, 0) && u(2, 0) && u(3, 0),
        4 => u(0, 0) && u(0, 1) && u(1, 0) && u(1, 1),
        _ => panic!("shape {a}"),
    }
}*/

pub fn solve(input: String, rocks: usize) -> usize {
    let input = input.trim();
    let mut f: Vec<Vec<bool>> = vec![];

    let shape = |a, u: &dyn Fn(usize, usize)| {
        match a {
            0 => {
                u(0, 0);
                u(0, 1);
                u(0, 2);
                u(0, 3);
            }
            1 => {
                u(0, 1);
                u(1, 0);
                u(1, 1);
                u(1, 2);
                u(2, 1);
            }
            2 => {
                u(0, 0);
                u(0, 1);
                u(0, 2);
                u(1, 2);
                u(2, 2);
            }
            3 => {
                u(0, 0);
                u(1, 0);
                u(2, 0);
                u(3, 0);
            }
            4 => {
                u(0, 0);
                u(0, 1);
                u(1, 0);
                u(1, 1);
            }
            _ => panic!("shape {a}"),
        }
    };

    let test = |si: usize, i0: usize, j0: usize| -> bool {
        if i0 < 0 { return false; }
        let mut ok = true;
        shape(si, &|di, dj| {
            let i = i0 + di;
            let j = j0 + dj;
            if j > 6 {
                ok = false;
            } else {
                if i < f.len() && f[i][j] {
                    ok = false;
                }
            }
        });
        ok
    };

    let put = |si, i0, j0| {
        shape(si, &|di, dj| {
            let i = i0 + di;
            let j = j0 + dj;
            while i > f.len() {
                f.push(vec![false; 7]);
            }
            f[i][j] = true;
        });
    };

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
        let input = util::read_file("inputs/day17-sample.txt");
        assert_eq!(3068, solve(input, 2022));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/day17.txt");
        assert_eq!(3106, solve(input, 2022));
    }

    //#[test]
    //fn part2_sample() {
    //    let input = util::read_file("inputs/day17-sample.txt");
    //    assert_eq!(56000011, solve(input, 1000000000000));
    //}

    //#[test]
    //fn part2_input() {
    //    let input = util::read_file("inputs/day17.txt");
    //    assert_eq!(12817603219131, solve(input, 1000000000000));
    //}
}
