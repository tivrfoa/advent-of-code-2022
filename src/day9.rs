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

// TODO create function that receives dir and just move 1 step
// it will be called in a loop and must return the new positions for H and T

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

        if hr == tr && hc == tc {
            // H covers T
            match dir {
                'R' => {
                    hc += 1;
                    if steps >= 2 {
                        hc += 1;
                        tc = hc - 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    for s in 2..steps {
                        hc += 1;
                        tc += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'L' => {
                    hc -= 1;
                    if steps >= 2 {
                        hc -= 1;
                        tc = hc + 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    for s in 2..steps {
                        hc -= 1;
                        tc -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'D' => {
                    hr += 1;
                    if steps >= 2 {
                        hr += 1;
                        tr = hr - 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    for s in 2..steps {
                        hr += 1;
                        tr += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'U' => {
                    hr -= 1;
                    if steps >= 2 {
                        hr -= 1;
                        tr = hr + 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    for s in 2..steps {
                        hr -= 1;
                        tr -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                d @ _ => panic!("Invalid dir: {d}"),
            }
        } else if hr != tr && hc != tc {
            // check diagonal
            match dir {
                'R' => {
                    if hc < tc {
                        // only move H. They will be in the same col
                        hc += 1;
                    } else {
                        hc += 1;
                        tc = hc - 1;
                        tr = hr;
                        ans += visit(&mut visited, tr, tc);
                    }

                    // now they might be in same colum and different row
                    if steps >= 2 {
                        if hc == tc {
                            // only move H
                            hc += 1;
                        } else {
                            hc += 1;
                            tc += 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    // might still be in diagonal
                    if steps >= 3 {
                        if hr != tr {
                            tr = hr;
                            hc += 1;
                            tc = hc - 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hc += 1;
                            tc += 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    for s in 3..steps {
                        hc += 1;
                        tc += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'L' => {
                    if hc > tc {
                        // only move H. They will be in the same col
                        hc -= 1;
                    } else {
                        hc -= 1;
                        tc = hc + 1;
                        tr = hr;
                        ans += visit(&mut visited, tr, tc);
                    }

                    // now they might be in same colum and different row
                    if steps >= 2 {
                        if hc == tc {
                            // only move H
                            hc -= 1;
                        } else {
                            hc -= 1;
                            tc -= 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    // might still be in diagonal
                    if steps >= 3 {
                        if hr != tr {
                            tr = hr;
                            hc -= 1;
                            tc = hc + 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hc -= 1;
                            tc -= 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    for s in 3..steps {
                        hc -= 1;
                        tc -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'D' => {
                    if hr > tr {
                        // only move H. They will be in the same row
                        hr += 1;
                    } else {
                        hr += 1;
                        tr = hr - 1;
                        tc = hc;
                        ans += visit(&mut visited, tr, tc);
                    }

                    // now they might be in same row and different col
                    if steps >= 2 {
                        if hc == tc {
                            // only move H
                            hr += 1;
                        } else {
                            hr += 1;
                            tr += 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    // might still be in diagonal
                    if steps >= 3 {
                        if hc != tc {
                            tc = hc;
                            hr += 1;
                            tr = hr - 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hr += 1;
                            tr += 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    for s in 3..steps {
                        hr += 1;
                        tr += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'U' => {
                    if hr < tr {
                        // only move H. They will be in the same row
                        hr -= 1;
                    } else {
                        hr -= 1;
                        tr = hr + 1;
                        tc = hc;
                        ans += visit(&mut visited, tr, tc);
                    }

                    // now they might be in same row and different col
                    if steps >= 2 {
                        if hc == tc {
                            // only move H
                            hr -= 1;
                        } else {
                            hr -= 1;
                            tr -= 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    // might still be in diagonal
                    if steps >= 3 {
                        if hc != tc {
                            tc = hc;
                            hr -= 1;
                            tr = hr + 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hr -= 1;
                            tr -= 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }

                    for s in 3..steps {
                        hr -= 1;
                        tr -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                d @ _ => panic!("Invalid dir: {d}"),
            }
        } else {
            match dir {
                'R' => {
                    if hr != tr {
                        // just change H col in first step
                        hc += 1;
                    } else {
                        hc += 1;
                        tc += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    if steps >= 2 {
                        if hr != tr {
                            // in the second step T must go to the same row
                            hc += 1;
                            tr = hr;
                            tc = hc - 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hc += 1;
                            tc += 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }
                    for s in 2..steps {
                        hc += 1;
                        tc += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'L' => {
                    if hr != tr {
                        // just change H col in first step
                        hc -= 1;
                    } else {
                        hc -= 1;
                        tc -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    if steps >= 2 {
                        if hr != tr {
                            // in the second step T must go to the same row
                            hc -= 1;
                            tr = hr;
                            tc = hc + 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hc -= 1;
                            tc -= 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }
                    for s in 2..steps {
                        hc -= 1;
                        tc -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'D' => {
                    // down row actually increases ...
                    if hc != tc {
                        // just change H row in first step
                        hr += 1;
                    } else {
                        hr += 1;
                        tr += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    if steps >= 2 {
                        if hc != tc {
                            // in the second step T must go to the same col
                            tc = hc;
                            hr += 1;
                            tr = hr - 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hr += 1;
                            tr += 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }
                    for s in 2..steps {
                        hr += 1;
                        tr += 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                'U' => {
                    // up row actually decreases ...
                    if hc != tc {
                        // just change H row in first step
                        hr -= 1;
                    } else {
                        hr -= 1;
                        tr -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                    if steps >= 2 {
                        if hc != tc {
                            // in the second step T must go to the same col
                            tc = hc;
                            hr -= 1;
                            tr = hr + 1;
                            ans += visit(&mut visited, tr, tc);
                        } else {
                            hr -= 1;
                            tr -= 1;
                            ans += visit(&mut visited, tr, tc);
                        }
                    }
                    for s in 2..steps {
                        hr -= 1;
                        tr -= 1;
                        ans += visit(&mut visited, tr, tc);
                    }
                }
                d @ _ => panic!("Invalid dir: {}", d),
            }
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
        assert_eq!(574080, solve(input)); // got 6846 which is wrong
    }

    // #[test]
    // fn part2_sample() {
    // 	let input = util::read_file("inputs/sample-day9.txt");
    // 	assert_eq!(8, solve(input));
    // }
}
