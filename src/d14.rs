use std::cmp::Ordering;
use std::{fmt::write, fs};

pub fn solve() {
    let path = "resources/d14.txt";
    let contents = fs::read_to_string(path).unwrap();
    let lines = contents.lines();
    let panel: Vec<Vec<char>> = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    println!("{}", contents);

    assert!(panel.iter().map(|v| v.len()).all(|i| i == panel[0].len()));

    // push all round rocks up
    let rock_locations = panel.iter().enumerate().fold(vec![], |mut locs, (row, v)| {
        locs.extend(v.iter().enumerate().filter_map(|(col, c)| {
            if *c == 'O' {
                Some((row, col))
            } else {
                None
            }
        }));
        return locs;
    });
    println!("rock counts:\n{:?}", rock_locations);

    let no_round_rocks = panel
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| match c {
                    'O' => '.',
                    '#' => '#',
                    '.' => '.',
                    _ => panic!(),
                })
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    let panel_to_string = |panel: &Vec<Vec<char>>| -> String {
        panel.iter().fold(String::from(""), |mut s, v| {
            s.push_str(&v.iter().fold(String::from("\n"), |mut s: String, c| {
                s.push(*c);
                return s;
            }));
            return s;
        })
    };
    println!("{}", panel_to_string(&no_round_rocks));

    let tilted = cycle(1000000000, no_round_rocks, rock_locations);
    println!("{}", panel_to_string(&tilted));

    let load = tilted
        .iter()
        .rev()
        .enumerate()
        .fold(0, |load_acc, (rev_row_ind, tilted_row)| {
            let rock_ct = tilted_row
                .iter()
                .fold(0, |rocks, c| rocks + if *c == 'O' { 1 } else { 0 });
            return load_acc + rock_ct * (rev_row_ind + 1);
        });
    println!("{}", load);
}

type Search = fn(&Vec<Vec<char>>, (usize, usize)) -> bool;
type Comp = dyn FnMut(&(usize, usize), &(usize, usize)) -> Ordering;

fn cycle(
    ncycles: usize,
    mut panel: Vec<Vec<char>>,
    mut rock_locations: Vec<(usize, usize)>,
) -> Vec<Vec<char>> {
    let comps: Vec<fn(&(usize, usize), &(usize, usize)) -> Ordering> = vec![
        |l: &(usize, usize), r: &(usize, usize)| l.0.partial_cmp(&r.0).unwrap(),
        |l: &(usize, usize), r: &(usize, usize)| l.1.partial_cmp(&r.1).unwrap(),
        |l: &(usize, usize), r: &(usize, usize)| r.0.partial_cmp(&l.0).unwrap(),
        |l: &(usize, usize), r: &(usize, usize)| r.1.partial_cmp(&l.1).unwrap(),
    ];
    let transforms: Vec<(Search, fn(usize, usize) -> (usize, usize))> = vec![
        (
            // North
            |panel: &Vec<Vec<char>>, (row, col): (usize, usize)| {
                row > 0 && panel[row - 1][col] == '.'
            },
            |row: usize, col: usize| (row - 1, col),
        ),
        (
            // West
            |panel: &Vec<Vec<char>>, (row, col): (usize, usize)| {
                col > 0 && panel[row][col - 1] == '.'
            },
            |row: usize, col: usize| (row, col - 1),
        ),
        (
            // South
            |panel: &Vec<Vec<char>>, (row, col): (usize, usize)| {
                row < panel.len() - 1 && panel[row + 1][col] == '.'
            },
            |row: usize, col: usize| (row + 1, col),
        ),
        (
            // East
            |panel: &Vec<Vec<char>>, (row, col): (usize, usize)| {
                col < panel[0].len() - 1 && panel[row][col + 1] == '.'
            },
            |row: usize, col: usize| (row, col + 1),
        ),
    ];

    let panel_to_string = |panel: &Vec<Vec<char>>| -> String {
        panel.iter().fold(String::from(""), |mut s, v| {
            s.push_str(&v.iter().fold(String::from("\n"), |mut s: String, c| {
                s.push(*c);
                return s;
            }));
            return s;
        })
    };

    let mut rock_location_map: Vec<Vec<(usize, usize)>> = vec![];
    rock_location_map.push(rock_locations.clone());
    for cyc in 0..ncycles {
        for (i, transform) in transforms.iter().enumerate() {
            rock_locations.sort_by(comps[i]);
            panel = rock_locations
                .iter()
                .fold(panel, |mut panel, (mut row, mut col)| {
                    // push the rock up to no_round_rocksthe top of the panel
                    while transform.0(&panel, (row, col)) {
                        panel[row][col] = '.';
                        (row, col) = transform.1(row, col);
                    }
                    panel[row][col] = 'O';
                    return panel;
                });
            rock_locations = panel.iter().enumerate().fold(vec![], |mut locs, (row, v)| {
                locs.extend(v.iter().enumerate().filter_map(|(col, c)| {
                    if *c == 'O' {
                        Some((row, col))
                    } else {
                        None
                    }
                }));
                return locs;
            });
            //println!("Inner cycle #{}:\n{}", cyc + 1, panel_to_string(&panel));
        }
        println!("After #{} cycle(s):{}", cyc + 1, panel_to_string(&panel));
        for (ind, loc2) in rock_location_map.iter().enumerate() {
            if locs_equal(&rock_locations, loc2) {
                println!("cycle found, go to index: {}", ind);
                let new_panel: Vec<Vec<char>> = panel
                    .iter()
                    .map(|v| v.iter().map(|c| if *c == 'O' { '.' } else { *c }).collect())
                    .collect();
                return rock_location_map[((ncycles - ind) % (cyc + 1 - ind)) + ind]
                    .iter()
                    .fold(new_panel, |mut panel, (row, col)| {
                        panel[*row][*col] = 'O';
                        panel
                    });
            }
        }
        rock_location_map.push(rock_locations.clone());
    }
    panel
}

fn tilt_north(panel: Vec<Vec<char>>, rock_locations: Vec<(usize, usize)>) -> Vec<Vec<char>> {
    rock_locations
        .iter()
        .fold(panel, |mut panel, (mut row, col)| {
            // push the rock up to no_round_rocksthe top of the panel
            while row > 0 && panel[row - 1][*col] == '.' {
                row -= 1;
            }
            panel[row][*col] = 'O';
            return panel;
        })
}

fn locs_equal(loc1: &Vec<(usize, usize)>, loc2: &Vec<(usize, usize)>) -> bool {
    for loc in loc1.iter() {
        if !loc2.contains(&loc) {
            return false;
        }
    }
    for loc in loc2.iter() {
        if !loc1.contains(&loc) {
            return false;
        }
    }
    return true;
}
