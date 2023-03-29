use crate::util;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// Day 7 using Rc<RefCell>
type RefDir<'a> = Rc<RefCell<Dir<'a>>>;

#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    size: usize,
    parent: Option<RefDir<'a>>,
    child_dirs: HashMap<&'a str, RefDir<'a>>,
}

impl<'a> Dir<'a> {
    fn new(name: &'a str, parent: Option<RefDir<'a>>) -> Self {
        Self {
            name,
            size: 0,
            parent,
            child_dirs: HashMap::new(),
        }
    }
}

pub fn solve(input: String) -> usize {
    let root = Rc::new(RefCell::new(Dir::new("/", None)));
    let mut curr_dir = root.clone();

    for line in input.lines().skip(1) {
        if line.starts_with("$ cd") {
            let dir = line.split_ascii_whitespace().nth(2).unwrap();

            if dir == ".." {
                let size = curr_dir.borrow().size; // adds up the size of child dir
                let tmp = curr_dir.borrow().parent.as_ref().unwrap().clone();
                curr_dir = tmp;
                curr_dir.borrow_mut().size += size;
            } else {
                let tmp = Rc::clone(
                    curr_dir
                        .borrow()
                        .child_dirs
                        .get::<str>(dir)
                        .as_ref()
                        .unwrap(),
                );
                curr_dir = tmp;
            }
        } else if line.starts_with("$ ls") {
            // just skip
        } else if line.starts_with("dir ") {
            let dir = line.split_ascii_whitespace().nth(1).unwrap();
            let parent = Some(curr_dir.clone());
            curr_dir
                .borrow_mut()
                .child_dirs
                .insert(dir, Rc::new(RefCell::new(Dir::new(dir, parent))));
        } else {
            let size: usize = line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap();
            curr_dir.borrow_mut().size += size;
        }
    }

    // I need to compute to go back to root (/) to add
    // child dirs sizes
    loop {
        if curr_dir.borrow().parent.is_none() {
            break;
        }
        let size = curr_dir.borrow().size;
        let tmp = curr_dir.borrow().parent.as_ref().unwrap().clone();
        curr_dir = tmp;
        curr_dir.borrow_mut().size += size;
    }

    let space_available: usize = 70000000 - root.borrow().size;
    const GOAL: usize = 30000000;
    let mut min = usize::MAX;

    let mut to_visit = vec![root];
    while let Some(rc) = to_visit.pop() {
        let size = rc.borrow().size;
        if size < min && space_available + size >= GOAL {
            min = size;
        }
        for rc2 in rc.borrow().child_dirs.values() {
            to_visit.push(rc2.clone());
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/2022/sample-day7.txt");
        assert_eq!(24933642, solve(input));
    }

    #[test]
    fn part1_input() {
        let input = util::read_file("inputs/2022/input-day7.txt");
        assert_eq!(7991939, solve(input));
    }
}
