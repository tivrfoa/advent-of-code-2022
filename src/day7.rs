use crate::util;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

// Day 7 using Rc<RefCell>
type RefDir = Rc<RefCell<Dir>>;

#[derive(Debug)]
struct Dir {
    name: String,
    size: usize,
	parent: Option<RefDir>,
    child_dirs: HashMap<String, RefDir>,
}

impl Dir {
	// TODO test name as &str
    fn new(name: String, parent: Option<RefDir>) -> Self {
        Self {
            name,
            size: 0,
			parent,
            child_dirs: HashMap::new(),
        }
    }
}

pub fn solve(input: String) -> u32 {

    let mut root = Rc::new(RefCell::new(Dir::new("/".into(), None)));
    let mut curr_dir = root.clone();
    let mut curr_size = 0;

    for line in input.lines().skip(1) {
        if line.starts_with("$ cd") {
            curr_size = 0;
            let dir = line.split_ascii_whitespace().skip(2).next().unwrap();

            if dir == ".." {
                let size = curr_dir.borrow().size; // adds up the size of child dir
				let tmp = curr_dir.borrow().parent.as_ref().unwrap().clone();
				curr_dir = tmp;
                curr_dir.borrow_mut().size += size;
            } else {
                let len = curr_dir.borrow().child_dirs.len();
				let tmp = Rc::clone(curr_dir.borrow().child_dirs.get::<String>(&dir.into()).as_ref().unwrap());
				curr_dir = tmp;
            }
        } else if line.starts_with("$ ls") {
            // just skip
        } else if line.starts_with("dir ") {
            // let dir = line.split_ascii_whitespace().skip(1).next().unwrap();
			// let parent = Some(curr_dir.clone());
			// curr_dir.child_dirs.insert(dir, Dir::new(
            // dirs.push(Dir::new(dir.into()));
            // let len = dirs.len();
            // dirs[curr_dir_index].child_dirs.push(len - 1);
        } else {
            // let size: usize = line
            //     .split_ascii_whitespace()
            //     .next()
            //     .unwrap()
            //     .parse()
            //     .unwrap();
            // dirs[curr_dir_index].size += size;
        }
    }

    // I need to compute all remaining prev_dir_index_stack to
    // add child dirs sizes
    //for parent_index in prev_dir_index_stack.iter().rev() {
    //    let size = dirs[curr_dir_index].size; // adds up the size of child dir
    //    curr_dir_index = *parent_index;
    //    dirs[curr_dir_index].size += size;
    //}

    //let space_available: usize = 70000000 - dirs[0].size;
    //const GOAL: usize = 30000000;
    //let mut min = dirs[0].size;

    //for dir in dirs.iter() {
    //    if dir.size < min && space_available + dir.size >= GOAL {
    //        min = dir.size;
    //    }
    //}

    //println!("min dir to delete: {}", min);
	0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = util::read_file("inputs/sample-day7.txt");
        assert_eq!(13140, solve(input));
    }

    #[test]
	#[ignore]
    fn part1_input() {
        let input = util::read_file("inputs/input-day7.txt");
        assert_eq!(14060, solve(input));
    }
}
