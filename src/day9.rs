use crate::util;

const R: usize = 1000;
const C: usize = 1000;

fn visit(visited: &mut [[bool; C]; R], tr: usize, tc: usize) -> u32 {
    if visited[tr][tc] {
        0
    } else {
        visited[tr][tc] = true;
        1
    }
}

fn move_dir(dir: char, hr: usize, hc: usize, tr: usize, tc: usize) -> (usize, usize, usize, usize) {
    match dir {
        'R' => {
            if hr == tr && hc == tc {
                // H covers T
                (hr, hc + 1, tr, tc)
            } else if hr != tr && hc != tc {
                // they are in diagonal
                if hc < tc {
                    (hr, hc + 1, tr, tc)
                } else {
                    // hc > tc
                    (hr, hc + 1, hr, hc)
                }
            } else {
                if hr != tr {
                    // same column, different row => only H moves
                    (hr, hc + 1, tr, tc)
                } else if hc + 1 == tc {
                    (hr, hc + 1, tr, tc)
                } else {
                    (hr, hc + 1, tr, tc + 1)
                }
            }
        }
        'L' => {
            if hr == tr && hc == tc {
                // H covers T
                (hr, hc - 1, tr, tc)
            } else if hr != tr && hc != tc {
                // they are in diagonal
                if hc > tc {
                    (hr, hc - 1, tr, tc)
                } else {
                    // hc < tc
                    (hr, hc - 1, hr, hc)
                }
            } else {
                if hr != tr {
                    // same column, different row => only H moves
                    (hr, hc - 1, tr, tc)
                } else if hc - 1 == tc {
                    (hr, hc - 1, tr, tc)
                } else {
                    (hr, hc - 1, tr, tc - 1)
                }
            }
        }
        'D' => {
            if hr == tr && hc == tc {
                // H covers T
                (hr + 1, hc, tr, tc)
            } else if hr != tr && hc != tc {
                // they are in diagonal
                if hr < tr {
                    (hr + 1, hc, tr, tc) // they'll be in same row
                } else {
                    // hr > tr
                    (hr + 1, hc, hr, hc)
                }
            } else {
                if hc != tc {
                    // same row, different column => only H moves
                    (hr + 1, hc, tr, tc)
                } else if hr + 1 == tr {
                    (hr + 1, hc, tr, tc)
                } else {
                    (hr + 1, hc, tr + 1, tc)
                }
            }
        }
        'U' => {
            if hr == tr && hc == tc {
                // H covers T
                (hr - 1, hc, tr, tc)
            } else if hr != tr && hc != tc {
                // they are in diagonal
                if hr > tr {
                    (hr - 1, hc, tr, tc) // they'll be in same row
                } else {
                    // hr < tr
                    (hr - 1, hc, hr, hc)
                }
            } else {
                if hc != tc {
                    // same row, different column => only H moves
                    (hr - 1, hc, tr, tc)
                } else if hr - 1 == tr {
                    (hr - 1, hc, tr, tc)
                } else {
                    (hr - 1, hc, tr - 1, tc)
                }
            }
        }
        d @ _ => panic!("Invalid dir: {d}"),
    }
}

pub fn solve(input: String) -> u32 {
    let mut ans = 0;
    let mut visited = [[false; C]; R];
    let mut hr = 500;
    let mut hc = 500;
    let mut tr = 500;
    let mut tc = 500;
    visited[tr][tc] = true;
    ans += 1;

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        let dir = tokens[0].chars().next().unwrap();
        let steps: usize = tokens[1].parse().unwrap();

        for _ in 0..steps {
            let (new_hr, new_hc, new_tr, new_tc) = move_dir(dir, hr, hc, tr, tc);
            hr = new_hr;
            hc = new_hc;
            tr = new_tr;
            tc = new_tc;
            ans += visit(&mut visited, tr, tc);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/sample-day9.txt");
        assert_eq!(13, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/input-day9.txt");

        // failed answers: 6846, 6821
        assert_eq!(574080, solve(input)); // got 6846 which is wrong
    }

    // #[test]
    // fn part2_sample() {
    // 	let input = util::read_file("inputs/sample-day9.txt");
    // 	assert_eq!(8, solve(input));
    // }
}
