use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("resources/d13.txt").unwrap();
    let _mat: Vec<Vec<char>> = vec![];
    for line in contents.lines() {
        let _sp = line.split_ascii_whitespace().into_iter();
        break;
    }
}
