use std::env;
use std::fs;

#[derive(Debug)]
struct Dir {
    name: String,
    size: usize,
    dirs_index: Vec<usize>,
}

impl Dir {
    fn new(name: String) -> Self {
        Self {
            name,
            size: 0,
            dirs_index: vec![],
        }
    }
}

fn main() {
    let contents = get_file_contents();

    let mut dirs: Vec<Dir> = vec![];
    let mut root = Dir {
        name: "/".into(),
        size: 0,
        dirs_index: vec![],
    };
    dirs.push(root);

    let mut curr_size = 0;
    let mut curr_dir_index = 0;
    let mut prev_dir_index_stack = vec![];

    for line in contents.lines().skip(1) {
        if line.starts_with("$ cd") {
            curr_size = 0;
            let dir = line.split_ascii_whitespace().skip(2).next().unwrap();

            if dir == ".." {
                let size = dirs[curr_dir_index].size; // adds up the size of child dir
                curr_dir_index = prev_dir_index_stack.pop().unwrap();
                dirs[curr_dir_index].size += size;
            } else {
                prev_dir_index_stack.push(curr_dir_index);
                let len = dirs[curr_dir_index].dirs_index.len();
                for i in 0..len {
                    let child_index = dirs[curr_dir_index].dirs_index[i];
                    if dirs[child_index].name == dir {
                        curr_dir_index = child_index;
                        break;
                    }
                }
            }
        } else if line.starts_with("$ ls") {
            // just skip?
        } else if line.starts_with("dir ") {
            let dir = line.split_ascii_whitespace().skip(1).next().unwrap();
            dirs.push(Dir::new(dir.into()));
            let len = dirs.len();
            dirs[curr_dir_index].dirs_index.push(len - 1);
        } else {
            let size: usize = line
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap();
            dirs[curr_dir_index].size += size;
        }
    }

    // I need to compute all remaining prev_dir_index_stack to
    // add child dirs sizes
    for parent_index in prev_dir_index_stack.iter().rev() {
        let size = dirs[curr_dir_index].size; // adds up the size of child dir
        curr_dir_index = *parent_index;
        dirs[curr_dir_index].size += size;
    }

    let space_available: usize = 70000000 - dirs[0].size;
    const GOAL: usize = 30000000;
    let mut min = dirs[0].size;

    for dir in dirs.iter() {
        if dir.size < min && space_available + dir.size >= GOAL {
            min = dir.size;
        }
    }

    println!("min dir to delete: {}", min);
}

fn get_file_contents() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
