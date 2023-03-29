// https://github.com/elizarov/AdventOfCode2022/blob/main/src/Day17.kt

use crate::util;

use std::collections::HashMap;

fn shape(a: usize, mut u: impl FnMut(usize, usize)) {
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
}

fn put(f: &mut Vec<Vec<bool>>, si: usize, i0: usize, j0: usize) {
    shape(si, |di, dj| {
        let i = i0 + di;
        let j = j0 + dj;
        while i + 1 > f.len() {
            f.push(vec![false; 7]);
        }
        f[i][j] = true;
    });
}

fn test(f: &[Vec<bool>], si: usize, i0: usize, j0: usize) -> bool {
    if i0 < 0 {
        return false;
    }
    let mut ok = true;
    shape(si, |di, dj| {
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
}

fn mask(f: &[Vec<bool>]) -> usize {
    let mut m = 0;
    let mut k = 1;
    for i in 0..7 {
        for j in 0..6 {
            if f.len() > 0 {
                let last_index = f.len() - 1;
                if i > last_index || f[last_index - i][j] {
                    m = m | k;
                }
            }
            k <<= 1;
        }
    }
    m
}

pub fn solve(input: String, rocks: usize) -> usize {
    let jets: Vec<char> = input.trim().chars().collect();
    let mut f: Vec<Vec<bool>> = vec![];

    let mut pj = 0;
    let mut sn = 5;
    let mut si = sn - 1;

    let mut us: Vec<Upd> = vec![];
    let mut ui: HashMap<Upd, usize> = HashMap::new();
    let mut ans = 0;

    let mut rec = |u: Upd| -> bool {
        let rn = us.len();
        let prev = *match ui.get(&u) {
            Some(idx) => idx,
            None => {
                ui.insert(u, rn);
                us.push(u.clone());
                return false;
            }
        };
        let clen = rn - prev;
        let rem = rocks - rn;
        let mut ds0 = 0;
        let i_rest = prev + (rem % clen);
        for k in 0..rn {
            ds0 += us[k].ds;
        }
        for k in prev..i_rest {
            ds0 += us[k].ds;
        }
        let mut ds_cycle = 0;
        for k in prev..rn {
            ds_cycle += us[k].ds;
        }
        ans = ds0 + (rem / clen) * ds_cycle;
        true
    };

    for rn in 0..rocks {
        si = (si + 1) % sn;
        let s0 = f.len();
        let mut i = s0 + 3;
        let mut j = 2;
        assert!(test(&f, si, i, j));
        loop {
            let d = jets[pj];
            pj = (pj + 1) % jets.len();
            match d {
                '<' => {
                    if j > 0 && test(&f, si, i, j - 1) {
                        j -= 1;
                    }
                }
                '>' => {
                    if test(&f, si, i, j + 1) {
                        j += 1;
                    }
                }
                _ => panic!("{d}"),
            };
            if i == 0 || !test(&f, si, i - 1, j) {
                break;
            }
            i -= 1;
        }
        let m1 = mask(&f);
        put(&mut f, si, i, j);
        if rec(Upd {
            m1,
            si,
            pj,
            ds: f.len() - s0,
            i: s0 - i,
            j,
        }) {
            break;
        }
    }

    ans
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Upd {
    m1: usize,
    si: usize,
    pj: usize,
    ds: usize,
    i: usize,
    j: usize,
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
        let input = util::read_file("inputs/2022/day17-sample.txt");
        assert_eq!(3068, solve(input, 2022));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/day17.txt");
        assert_eq!(3106, solve(input, 2022));
    }

    #[test]
    fn part2_sample() {
        let input = util::read_file("inputs/2022/day17-sample.txt");
        assert_eq!(1514285714288, solve(input, 1000000000000));
    }

    #[test]
    fn part2_input() {
        let input = util::read_file("inputs/2022/day17.txt");
        assert_eq!(1537175792495, solve(input, 1000000000000));
    }
}
