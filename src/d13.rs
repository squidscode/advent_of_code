use std::fs;
use std::process::exit;

#[derive(Debug,Copy,Clone,PartialEq)]
enum Rtype {
    Vertical,
    Horizontal
}

pub fn solve() {
    let contents = fs::read_to_string("resources/d13.txt").unwrap();
    let mut mat: Vec<Vec<char>> = vec![];
    let mut solution = 0;
    for line in contents.lines() {
        if line != "" {
            println!("{}", line);
            mat.push(line.chars().collect());
        } else {
            let (t, ans) = find_new_reflection(mat);
            println!("{:?} {:?}\n", t, ans);
            match t {
                Rtype::Vertical => solution += ans,
                Rtype::Horizontal => solution += 100*ans,
            }
            mat = vec![];
        }
    }
    let (t, ans) = find_new_reflection(mat);
    println!("{:?} {:?}\n", t, ans);
    match t {
        Rtype::Vertical => solution += ans,
        Rtype::Horizontal => solution += 100*ans,
    }
    println!("Solution: {solution}")
}

fn find_new_reflection(mat: Vec<Vec<char>>) -> (Rtype, usize) {
    let flip_char = |c| -> char{match c {
            '.' => '#',
            '#' => '.',
            _ => panic!()
        }
    };
    let psolution = find_reflections(mat.clone());
    println!("Original reflections: {:?}", psolution);
    assert!(psolution.len() == 1);
    let psolution = psolution[0];
    let mut solution = vec![];
    for row in 0..mat.len() {
        for col in 0..mat[0].len() {
            let mut mc = mat.clone();
            mc[row][col] = flip_char(mc[row][col]);
            let new_reflections = find_reflections(mc);
            // println!("flip({row}, {col}) => {:?}", new_reflections);
            new_reflections.into_iter().for_each(|(t,l)|{
                if (t,l) != psolution && !solution.contains(&(t,l)) {
                    solution.push((t,l));
                }
            });
        }
    }
    println!("solutions: {:?}", solution);
    assert!(solution.len() == 1);
    return solution[0];
}

// Brute force solution to finding a reflection line
fn find_reflections(mat: Vec<Vec<char>>) -> Vec<(Rtype, usize)> {
    let mut solutions = vec![];
    // over Vertical reflections 
    for flip in 1..mat[0].len() {
        let mut works = true;
        for row in 0..mat.len() {
            for diff in 0..std::cmp::min(flip, mat[0].len()-flip) {
                if mat[row][flip-diff-1] != mat[row][flip+diff] {
                    works = false;
                    break;
                }
            }
            if !works {
                break;
            }
        }
        if works {
            solutions.push((Rtype::Vertical, flip));
        }
    }

    // over Horizontal reflections
    for flip in 1..mat.len() {
        let mut works = true;
        for col in 0..mat[0].len() {
            for diff in 0..std::cmp::min(flip, mat.len()-flip) {
                if mat[flip-diff-1][col] != mat[flip+diff][col] {
                    works = false;
                    break;
                }
            }
            if !works {
                break;
            }
        }
        if works {
            solutions.push((Rtype::Horizontal, flip));
        }
    }
    solutions
}

